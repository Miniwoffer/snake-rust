//mod game {
    extern crate orbclient;

    use orbclient::{Color,Window,Renderer};

    static mut ID_COUNTER : i32 = 0;


    pub struct GameObject {
        pub x:i32,
        pub y:i32,
        pub w:u32,
        pub h:u32,
        pub color:Color,
        pub id : i32,

    }

    impl GameObject {
        pub fn new(x:i32,y:i32,w:u32,h:u32,r:u8,g:u8,b:u8) -> GameObject {
            unsafe{
            ID_COUNTER += 1;
            GameObject {x:x,y:y,w:w,h:h,color:Color::rgb(r,g,b),id:ID_COUNTER}
            }
        }
        pub fn intersects(&self , other: &GameObject) -> bool{
            if  self.x  +   (self.w as i32)  < other.x &&
                other.x +   (other.w as i32) < self.x  &&
                self.y  +   (self.h as i32) < other.y &&
                other.y +   (other.h as i32) < self.y &&
                other.id != self.id {
                    return true;
                }
            false
        }
        pub fn draw(&self, window : &mut Window)
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
