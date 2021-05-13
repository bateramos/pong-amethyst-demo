use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{transform::Transform},
    ecs::{World, Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, UiText, LineMode, TtfFormat, UiTransform},
};

use crate::audio::initialise_audio;

pub const AREA_HEIGHT: f32 = 100.0;
pub const AREA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 20.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS:f32 = 2.0;

#[derive(Debug)]
pub enum PongEvent {
    Bounce(&'static str), Score,
}

#[derive(Default)]
pub struct Pong {
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handler: Option<Handle<SpriteSheet>>,
}

pub struct SpriteSheetHandler {
    pub sprite_sheet_handler: Handle<SpriteSheet>,
}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handler = load_sprite_sheet(world);

        self.ball_spawn_timer.replace(1.0);
        self.sprite_sheet_handler.replace(sprite_sheet_handler.clone());

        initialise_paddles(world, self.sprite_sheet_handler.clone().unwrap());
        initialise_camera(world);
        initialise_scoreboard(world);
        initialise_audio(world);

        world.insert(SpriteSheetHandler { sprite_sheet_handler });
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(AREA_WIDTH * 0.5, AREA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(AREA_WIDTH, AREA_HEIGHT))
        .with(transform)
        .build();
}

pub enum Side {
    Left, Right
}

pub struct Paddle {
    pub side : Side,
    pub width : f32,
    pub height : f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle { side, width: PADDLE_WIDTH, height: PADDLE_HEIGHT }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = AREA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(AREA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .with(sprite_render.clone())
        .build();

    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .with(sprite_render)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "pong_spritesheet.png", ImageFormat::default(), (), &texture_storage
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "pong_spritesheet.ron", SpriteSheetFormat(texture_handle), (), &sprite_sheet_storage
    )
}

#[derive(Debug)]
pub struct Ball {
    pub id: &'static str,
    pub velocity: [f32; 2],
    pub original_velocity: [f32; 2],
    pub radius: f32,
}

impl Ball {
    pub fn default() -> Self {
        let velocity = [BALL_VELOCITY_X, BALL_VELOCITY_Y];
        Self { id: "ball", radius: BALL_RADIUS, velocity: velocity.clone(), original_velocity: velocity }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: f32,
    pub score_right: f32,
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

fn initialise_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        -50., -50., 1., 200., 50.,
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        50., -50., 1., 200., 50.,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    world.insert(ScoreText { p1_score, p2_score });
}
