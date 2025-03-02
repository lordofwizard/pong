use bevy::prelude::*;
use rand::random;

const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const BALL_SIZE: f32 = 15.0;
const PADDLE_SPEED: f32 = 500.0;
const BALL_SPEED: f32 = 400.0;
const WALL_THICKNESS: f32 = 10.0;

const BG_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const PADDLE_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const BALL_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const WALL_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const TEXT_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Component)]
struct Paddle {
    side: PaddleSide,
}

#[derive(PartialEq)]
enum PaddleSide {
    Left,
    Right,
}

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct ScoreText {
    side: PaddleSide,
}

#[derive(Resource)]
struct Score {
    left: u32,
    right: u32,
}

#[derive(Resource)]
struct WallLocations {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(Score { left: 0, right: 0 })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (paddle_movement, ball_movement, ball_collision, score_update),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.single();
    let window_width = window.width();
    let window_height = window.height();

    let top_wall_y = window_height / 2.0 - WALL_THICKNESS / 2.0;
    let bottom_wall_y = -window_height / 2.0 + WALL_THICKNESS / 2.0;
    let left_wall_x = -window_width / 2.0 + WALL_THICKNESS / 2.0;
    let right_wall_x = window_width / 2.0 - WALL_THICKNESS / 2.0;

    commands.insert_resource(WallLocations {
        top: top_wall_y,
        bottom: bottom_wall_y,
        left: left_wall_x,
        right: right_wall_x,
    });

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, top_wall_y, 0.0),
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(window_width, WALL_THICKNESS)),
                ..default()
            },
            ..default()
        },
        Collider,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, bottom_wall_y, 0.0),
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(window_width, WALL_THICKNESS)),
                ..default()
            },
            ..default()
        },
        Collider,
    ));

    let dash_count = 20;
    let dash_height = window_height / (dash_count * 2) as f32;
    for i in 0..dash_count {
        let y_pos = -window_height / 2.0 + (i as f32 * 2.0 + 1.0) * dash_height;
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(0.0, y_pos, 0.0),
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(2.0, dash_height)),
                ..default()
            },
            ..default()
        });
    }

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(left_wall_x + 30.0, 0.0, 0.0),
            sprite: Sprite {
                color: PADDLE_COLOR,
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..default()
            },
            ..default()
        },
        Paddle {
            side: PaddleSide::Left,
        },
        Collider,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(right_wall_x - 30.0, 0.0, 0.0),
            sprite: Sprite {
                color: PADDLE_COLOR,
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..default()
            },
            ..default()
        },
        Paddle {
            side: PaddleSide::Right,
        },
        Collider,
    ));

    let initial_direction = Vec2::new(0.7, 0.3).normalize();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                color: BALL_COLOR,
                custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
                ..default()
            },
            ..default()
        },
        Ball {
            velocity: initial_direction * BALL_SPEED,
        },
    ));

    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: TEXT_COLOR,
    };

    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::new("0", text_style.clone())],
                justify: JustifyText::Center,
                ..default()
            },
            transform: Transform::from_xyz(-50.0, window_height / 2.0 - 60.0, 0.0),
            ..default()
        },
        ScoreText {
            side: PaddleSide::Left,
        },
    ));

    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::new("0", text_style)],
                justify: JustifyText::Center,
                ..default()
            },
            transform: Transform::from_xyz(50.0, window_height / 2.0 - 60.0, 0.0),
            ..default()
        },
        ScoreText {
            side: PaddleSide::Right,
        },
    ));
}

fn paddle_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
    time: Res<Time>,
    wall_locations: Res<WallLocations>,
) {
    let delta = time.delta_seconds();

    for (paddle, mut transform) in query.iter_mut() {
        match paddle.side {
            PaddleSide::Left => {
                if keyboard_input.pressed(KeyCode::KeyA) {
                    transform.translation.y += PADDLE_SPEED * delta;
                }
                if keyboard_input.pressed(KeyCode::KeyZ) {
                    transform.translation.y -= PADDLE_SPEED * delta;
                }
            }
            PaddleSide::Right => {
                if keyboard_input.pressed(KeyCode::KeyJ) {
                    transform.translation.y += PADDLE_SPEED * delta;
                }
                if keyboard_input.pressed(KeyCode::KeyN) {
                    transform.translation.y -= PADDLE_SPEED * delta;
                }
            }
        }

        let half_paddle_height = PADDLE_HEIGHT / 2.0;
        let top_bound = wall_locations.top - half_paddle_height - WALL_THICKNESS / 2.0;
        let bottom_bound = wall_locations.bottom + half_paddle_height + WALL_THICKNESS / 2.0;
        transform.translation.y = transform.translation.y.clamp(bottom_bound, top_bound);
    }
}

fn ball_movement(mut ball_query: Query<(&mut Ball, &mut Transform)>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for (ball, mut transform) in ball_query.iter_mut() {
        transform.translation.x += ball.velocity.x * delta;
        transform.translation.y += ball.velocity.y * delta;
    }
}

fn ball_collision(
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    _collider_query: Query<(&Transform, &Sprite), With<Collider>>,
    paddle_query: Query<(&Transform, &Sprite, &Paddle)>,
    mut score: ResMut<Score>,
    wall_locations: Res<WallLocations>,
    mut commands: Commands,
) {
    let (mut ball, ball_transform, ball_sprite) = ball_query.single_mut();
    let ball_size = ball_sprite.custom_size.unwrap_or(Vec2::ONE);
    let ball_half_size = ball_size / 2.0;

    if ball_transform.translation.y + ball_half_size.y > wall_locations.top - WALL_THICKNESS / 2.0
        || ball_transform.translation.y - ball_half_size.y
            < wall_locations.bottom + WALL_THICKNESS / 2.0
    {
        ball.velocity.y = -ball.velocity.y;
    }

    if ball_transform.translation.x < wall_locations.left {
        score.right += 1;
        reset_ball(&mut ball, &mut commands, Vec2::new(-1.0, 0.0));
    } else if ball_transform.translation.x > wall_locations.right {
        score.left += 1;
        reset_ball(&mut ball, &mut commands, Vec2::new(1.0, 0.0));
    }

    for (paddle_transform, paddle_sprite, paddle) in paddle_query.iter() {
        let paddle_size = paddle_sprite.custom_size.unwrap_or(Vec2::ONE);
        let paddle_half_size = paddle_size / 2.0;

        if ball_transform.translation.x + ball_half_size.x
            > paddle_transform.translation.x - paddle_half_size.x
            && ball_transform.translation.x - ball_half_size.x
                < paddle_transform.translation.x + paddle_half_size.x
            && ball_transform.translation.y + ball_half_size.y
                > paddle_transform.translation.y - paddle_half_size.y
            && ball_transform.translation.y - ball_half_size.y
                < paddle_transform.translation.y + paddle_half_size.y
        {
            let offset = (ball_transform.translation.y - paddle_transform.translation.y)
                / paddle_half_size.y;
            let bounce_angle = offset * std::f32::consts::PI / 4.0;

            let direction = match paddle.side {
                PaddleSide::Left => 1.0,
                PaddleSide::Right => -1.0,
            };

            let speed_increase = 1.05;
            let new_speed = ball.velocity.length() * speed_increase;

            ball.velocity.x = direction * new_speed * bounce_angle.cos();
            ball.velocity.y = new_speed * bounce_angle.sin();
        }
    }
}

fn reset_ball(ball: &mut Ball, commands: &mut Commands, direction: Vec2) {
    commands.add(|world: &mut World| {
        for mut transform in world
            .query_filtered::<&mut Transform, With<Ball>>()
            .iter_mut(world)
        {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
        }
    });

    let random_y = (random::<f32>() - 0.5) * 0.5;
    ball.velocity = Vec2::new(direction.x, random_y).normalize() * BALL_SPEED;
}

fn score_update(score: Res<Score>, mut query: Query<(&ScoreText, &mut Text)>) {
    if score.is_changed() {
        for (score_text, mut text) in query.iter_mut() {
            match score_text.side {
                PaddleSide::Left => {
                    text.sections[0].value = score.left.to_string();
                }
                PaddleSide::Right => {
                    text.sections[0].value = score.right.to_string();
                }
            }
        }
    }
}
