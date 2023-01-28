use nalgebra::{Matrix4, Vector3};

pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub yaw: f32,
    pub pitch: f32,

    up: Vector3<f32>,
    front: Vector3<f32>,
    right: Vector3<f32>,
    world_up: Vector3<f32>,

    first_mouse: bool,
    last_x: f32,
    last_y: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 3.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            fov: 45.0,
            near: 0.1,
            far: 100.0,
            yaw: -90.0,
            pitch: 0.0,

            up: Vector3::new(0.0, 1.0, 0.0),
            front: Vector3::new(0.0, 0.0, -1.0),
            right: Vector3::new(1.0, 0.0, 0.0),
            world_up: Vector3::new(0.0, 1.0, 0.0),

            first_mouse: true,
            last_x: 0.0,
            last_y: 0.0,
        }
    }

    pub fn get_projection_matrix(&self, aspect_ratio: f32) -> Matrix4<f32> {
        Matrix4::new_perspective(aspect_ratio, self.fov, self.near, self.far)
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            &self.position.into(),
            &(self.position + self.front).into(),
            &self.up.into(),
        )
    }

    pub fn on_mouse_move(&mut self, xpos: f32, ypos: f32) {
        if self.first_mouse {
            self.last_x = xpos;
            self.last_y = ypos;
            self.first_mouse = false;
        }

        let xoffset = xpos - self.last_x;
        let yoffset = self.last_y - ypos;
        self.last_x = xpos;
        self.last_y = ypos;

        let sensitivity = 0.1;
        let xoffset = xoffset * sensitivity;
        let yoffset = yoffset * sensitivity;

        self.yaw += xoffset;
        self.pitch += yoffset;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }
    }

    pub fn update(&mut self) {
        let front = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = front.normalize();

        self.right = self.front.cross(&self.world_up).normalize();
        self.up = self.right.cross(&self.front).normalize();
    }

    pub fn move_forward(&mut self, amount: f32) {
        self.position += self.front * amount;
    }

    pub fn move_backward(&mut self, amount: f32) {
        self.position -= self.front * amount;
    }

    pub fn move_left(&mut self, amount: f32) {
        self.position -= self.right * amount;
    }

    pub fn move_right(&mut self, amount: f32) {
        self.position += self.right * amount;
    }
}
