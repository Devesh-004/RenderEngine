#[derive(Debug)]
enum Node{
    Element{
        tag:String,
        children:Vec<Node>,
    },
    Text(String),
}
impl Node {
    fn element(tag: &str, children: Vec<Node>) -> Node{
        Node::Element { tag: tag.to_string(), children, }
    }
    fn text(content: &str) -> Node{
        Node::Text(content.to_string())
    }
    fn print(&self, depth: usize){
        let indent=" ".repeat(depth);

        match self{
            Node::Element { tag, children }=>{
                println!("{}<{}>", indent, tag);

                for child in children{
                    child.print(depth + 1);
                }
                println!("{}</{}>", indent, tag);
            }

            Node::Text(content)=>{
                println!("{}\"{}\"",indent, content);
            }
        }
    }
}
fn main(){
    let dom= Node::element(
        "html",  
        vec![
            Node::element(
                "body", vec![
                    Node::element("h1", vec![
                        Node::text("This is a Para Heading")
                        ],
                    ),
                    Node::element("p", vec![
                        Node::text("This is the paragraph")
                        ],
                    ),
                ],
            ),
        ],
    );
    dom.print(0);
}