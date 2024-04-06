use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;
use std::ops::Index;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 { 
    pub e: [f32; 3],
}

// alias for Vec3 called Point3
// makes it easier to understand what the Vec3 is being used for in code
pub type Point3 = Vec3;

impl Vec3 { 
    pub fn x(self) -> f32 { return self.e[0] }
    pub fn y(self) -> f32 { return self.e[1] }
    pub fn z(self) -> f32 { return self.e[2] }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 { 
        Vec3 { e: [x, y, z] }
    }

    pub fn length(&self) -> f32 { 
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn length_squared(&self) -> f32 { 
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn normalize(&self) -> Vec3 { 
        let k = 1.0 / self.length();
        Vec3 { e: [self.e[0] * k, self.e[1] * k, self.e[2] * k] }
    }

    pub fn dot(&self, other: Vec3) -> f32 { 
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: Vec3) -> Vec3 { 
        Vec3 { 
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ]
        }
    }
    
    pub fn unit_vector(self) -> Vec3 { 
        self / self.length()
    }

    pub fn random() -> Vec3 { 
        Vec3 { 
            e: [
                rand::random::<f32>(),
                rand::random::<f32>(),
                rand::random::<f32>(),
            ]
        }
    }

    pub fn random_range(min: f32, max: f32) -> Vec3 { 
        Vec3 { 
            e: [
                rand::random::<f32>() * (max - min) + min,
                rand::random::<f32>() * (max - min) + min,
                rand::random::<f32>() * (max - min) + min,
            ]
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 { 
        loop { 
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 { return p; }
        }
    }

    pub fn random_unit_vector() -> Vec3 { 
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 { 
        let in_unit_sphere = Vec3::random_unit_vector();
        if in_unit_sphere.dot(normal) > 0.0 { 
            return in_unit_sphere;
        } else { 
            return -in_unit_sphere;
        }
    }

    pub fn reflect(&self, n: Vec3) -> Vec3 { 
        *self - n * self.dot(n) * 2.0
    }

    pub fn near_zero(&self) -> bool { 
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }
}



// negate vec
impl Neg for Vec3 { 
    type Output = Vec3;

    fn neg(self) -> Vec3 { 
        Vec3 { 
            e: [
                -self.e[0],
                -self.e[1],
                -self.e[2],
            ]
        }
    }
}

// divide vec by scalar
impl Div<f32> for Vec3 { 
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 { 
        Vec3 { 
            e: [
                self.e[0] / t,
                self.e[1] / t,
                self.e[2] / t,
            ]
        }
    }
}

// subtract vecs
impl Sub for Vec3 { 
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 { 
        Vec3 { 
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ]
        }
    }
}

//add vecs
impl Add for Vec3 { 
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 { 
        Vec3 { 
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ]
        }
    }
}

// multiply vec by scalar
impl Mul<f32> for Vec3 { 
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 { 
        Vec3 { 
            e: [
                self.e[0] * t,
                self.e[1] * t,
                self.e[2] * t,
            ]
        }
    }
}

impl Mul<Vec3> for f32 { 
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 { 
        Vec3 { 
            e: [
                v.e[0] * self,
                v.e[1] * self,
                v.e[2] * self,
            ]
        }
    }
}

// index vec
impl Index<usize> for Vec3 { 
    type Output = f32;

    fn index(&self, i: usize) -> &f32 { 
        &self.e[i]
    }
}

// write out vec
impl fmt::Display for Vec3 { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

