pub enum Collider {
    Point,
    Ellipse(pub Vec2),
    Rectangle(pub Vec2),
    ConvexPolygon(pub [Vec2]),
}

pub struct Collision {
    pub colliders: [Box<Collider>],
}
