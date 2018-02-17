
extern crate orbclientear

extern crate rand;

use orbclient::{Window,Renderer,EventOption};

use rand::distributions::{IndependentSample, Range};

use std::time::{SystemTime};

mod game;

use game::GameObject;

fn main() {
    //random stuff
    let betwene = Range::new(0,255);
    let mut rng = rand::thread_rng();


    let (width, height) = orbclient::get_display_size().unwrap();
    
    let flags = vec![orbclient::WindowFlag::Async];
    
    let mut window = Window::new_flags((width as i32)/4,
                                (height as i32)/4,
                                width/2,
                                height/2,
                                "snake",
                                &flags).unwrap();

    let mut snake : Vec<GameObject> = Vec::new();
    
    let mut dir = 0;
    let mut speed = 10;
    
    let mut time_since_last_draw = SystemTime::now();
     
    snake.push(GameObject::new(1,1,10,10,betwene.ind_sample(&mut rng),50,50));

    'events: loop {
        for event in window.events(){
            match event.to_option(){
                EventOption::Quit(_quit_event) => break 'events,
                EventOption::Key(evt) => {
                    match evt.scancode {
                        77 => dir = 0,
                        75 => dir = 1,
                        80 => dir = 2,
                        72 => dir = 3,
                        _ => {
                            snake.push(GameObject::new(1,1,10,10,
                                    betwene.ind_sample(&mut rng),
                                    50,
                                    50));
                            //println!("{:?}",evt);
                        }
                    }
                },
                
                event_option =>
                {
                    println!("{:?}",event_option);
                }
            }
        }
        if time_since_last_draw.elapsed().unwrap().subsec_nanos() > 100000000
        {

            time_since_last_draw = SystemTime::now();
        match dir {
            0 => {
                snake[0].x+=speed;
            },
            1 => {
                snake[0].x-=speed;
            },
            2 => {
                snake[0].y+=speed;
            },
            _ => {
                snake[0].y-=speed;
            }
        }

        window.clear();
        let mut last_x = -100;
        let mut last_y = 0;
        
        for a in &snake{
            for b in &snake{
                if a.intersects(b) {
                    println!("collision betwene {:?} and {:?}",a.id,b.id);
                }
            }
        }

        for parts in &mut snake {


            if last_x != -100 {
                let temp_x = parts.x;
                let temp_y = parts.y;

                parts.x = last_x;
                parts.y = last_y;

                last_x = temp_x;
                last_y = temp_y;
            }
            else {
                last_x = parts.x;
                last_y = parts.y;
            }
            parts.draw(&mut window);
            //window.circle(parts.x,parts.y,-speed/2,parts.color);
        }
            window.sync();
        }
    }
}
