use chrono::prelude::*;use crossterm::{cursor::*,execute,style::*,terminal::*}
;use std::io::stdout;fn main(){let n=[119,5,59,47,77,110,126,7,127,79];let s=[
(07,2),(0,2),(07,6),(0,6),(0,6),(0,10),(0,2)];let c=include_str!("main.rs");//
execute!(stdout(),EnterAlternateScreen,MoveTo(0,0),Print(c));let f=|x:u32,d://
u32|{for(i)in(0..7){if(n[(x)as(usize)]>>i)&1==1{let(x,y)=s[i];for(j)in(0..(if(
i&1==0){5}else{8})){let (x,y)=match(i&1){0=>(x,y+j),_=>(x+j,y)};let p=x+(d)as(
usize)+y*79;let c=&c[p..p+1];execute!(stdout(),MoveTo((x+(d)as(usize))as(u16),
(y)as(u16)),PrintStyledContent(c.red()));}}}};let d=|ex:u16,whyyy:u16,content:
&str|{execute!(stdout(),MoveTo(ex,whyyy),PrintStyledContent(content.red()));};
loop{execute!(stdout(),MoveTo(0,0),Print(c));let t=chrono::offset::Local::now(
).time();d(25,4,"(x");d(25,8,"ov");d(49,4,"et");d(49,8,"le");f(t.hour()/10,4);
f(t.hour()%10,14);f(t.minute()/10,28);f(t.minute()%10,38);f(t.second()/10,54);
f(t.second()%10,64);std::thread::sleep(std::time::Duration::from_secs(1));}}//