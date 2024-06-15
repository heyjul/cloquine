use chrono::prelude::*;use crossterm::{cursor::*,execute,style::*,terminal::*}
;use std::io::stdout;fn main(){let n=[119,5,59,47,77,110,126,7,127,79];let s=[
(13,2),(6,2),(13,6),(6,6),(6,6),(6,10),(6,2)];let c=include_str!("main.rs");//
execute!(stdout(),EnterAlternateScreen,MoveTo(0,0),Print(c));let f=|x:u32,d://
u32|{for(i)in(0..7){if(n[(x)as(usize)]>>i)&1==1{let(x,y)=s[i];for(j)in(0..(if(
i&1==0){5}else{8})){let (x,y)=match(i&1){0=>(x,y+j),_=>(x+j,y)};let p=x+(d)as(
usize)+y*79;let c=&c[p..p+1];execute!(stdout(),MoveTo((x+(d)as(usize))as(u16),
(y)as(u16)),PrintStyledContent(c.red()));}}}};let d=|ex:u16,igrec:u16,content:
&str|{execute!(stdout(),MoveTo(ex,igrec),PrintStyledContent(content.red()));};
loop{execute!(stdout(),MoveTo(0,0),Print(c));let t=chrono::offset::Local::now(
).time();d(27,4,")a");d(27,8,"eT");d(49,4,"et");d(49,8,"le");f(t.hour()/10,2);
f(t.hour()%10,12);f(t.minute()/10,24);f(t.minute()%10,34);f(t.second()/10,46);
f(t.second()%10,56);std::thread::sleep(std::time::Duration::from_secs(1));}}//