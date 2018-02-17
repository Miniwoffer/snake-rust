//mod game {
    extern crate orbclient;
    use orbclient::{Color,Window,Renderer};


    pub struct GameObject {
        pub x:i32,
        pub y:i32,
        pub w:u32,
        pub h:u32,
        pub color:Color,
    }

    impl GameObject {
        pub fn new(x:i32,y:i32,w:u32,h:u32,r:u8,g:u8,b:u8) -> GameObject {
            GameObject {x:x,y:y,w:w,h:h,color:Color::rgb(r,g,b)}
        }
        pub fn intersects(&self , other:GameObject) -> bool{
            if  self.x  +   (self.w as i32)  < other.x ||
                other.x +   (other.w as i32) < self.x  ||
                self.y  +   (self.h as i32) < other.y ||
                other.y +   (other.h as i32) < self.y {
                    return true;
                }
            false
        }
        pub fn draw(&self, window:Window)
        {
            window.rect(
                self.x,
                self.y,
                self.w,
                self.h,
                self.color,
                )
        }
    }
//}
