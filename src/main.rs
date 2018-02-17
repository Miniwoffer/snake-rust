
extern crate orbclient;

extern crate rand;

use orbclient::{Color,Window,Renderer,EventOption};

use rand::distributions::{IndependentSample, Range};

use std::time::{SystemTime};

mod game;
use game::GameObject;

static SIZE : u32 = 500;

fn main() {
    //random stuff
    let betwene = Range::new(0,255);
    let mut rng = rand::thread_rng();


    let (width, height) = orbclient::get_display_size().unwrap();
    
    let flags = vec![orbclient::WindowFlag::Async];
    
    let mut window = Window::new_flags((width as i32)/4,
                                (height as i32)/4,
                                SIZE,
                                SIZE,
                                "snake",
                                &flags).unwrap();

    let mut snake : Vec<GameObject> = Vec::new();
    
    
    let mut map   : Vec<GameObject> = Vec::new();
    
    let times = SIZE/10;
    let randpos = Range::new(1,times-2);
    for x in 0..times {
        map.push(GameObject::new((x as i32)*10,0,10,10,50,betwene.ind_sample(&mut rng),50));
        map.push(GameObject::new((x as i32)*10,(SIZE as i32)-10,10,10,50,betwene.ind_sample(&mut rng),50));
        map.push(GameObject::new(0,(x as i32)*10,10,12,50,betwene.ind_sample(&mut rng),50));
        map.push(GameObject::new((SIZE as i32)-10,(x as i32)*10,10,10,50,betwene.ind_sample(&mut rng),50));
    }


    let mut point : GameObject = GameObject::new(200,200,10,10,255,255,255);


    let mut dir = 0;
    let mut speed = 10;
    let mut points = 0;
    let mut time_since_last_draw = SystemTime::now();
     
    snake.push(GameObject::new(100,100,10,10,betwene.ind_sample(&mut rng),50,50));

    'events: loop {
        for event in window.events(){
            match event.to_option(){
                EventOption::Quit(_quit_event) => break 'events,
                EventOption::Key(evt) => {
                    if evt.pressed {
                    match evt.scancode {
                        77 => dir = 0,
                        75 => dir = 1,
                        80 => dir = 2,
                        72 => dir = 3,
                        _ => {
                             println!("{:?}",evt);
                        }
                    }
                    }
                },
                
                event_option =>
                {
                    println!("{:?}",event_option);
                }
            }
        }
        if time_since_last_draw.elapsed().unwrap().subsec_nanos() > 150000000
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
        let mut snakegrow = false;

        for a in &snake{
            //check col with point
            if a.intersects(&point) {
                snakegrow = true;
                loop {
                    point.x = 10*(randpos.ind_sample(&mut rng) as i32);
                    point.y = 10*(randpos.ind_sample(&mut rng) as i32);
                    let mut collides : bool = false;
                    for part in &snake {
                        if part.intersects(&point){
                            collides = true;
                        }
                    }
                    if !collides {
                        break;
                    }
                }
            }
            //check col with self
            for b in &snake{
                if a.intersects(b) {
                    println!("Game Over");
                }
            }
            //check col with map
            for b in &map {
                if a.intersects(b) {
                    println!("Game Over");
                }

            }
        }
        point.color = Color::rgb(betwene.ind_sample(&mut rng),betwene.ind_sample(&mut rng),betwene.ind_sample(&mut rng));
        point.draw(&mut window);
        if snakegrow {
            snake.push(GameObject::new(-10,-10,10,10,
                                    betwene.ind_sample(&mut rng),
                                    50,
                                    50));    
        }

        for blocks in &mut map {
            blocks.draw(&mut window);
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
