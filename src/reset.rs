//container-type to allow extender to works
//without x overflow when screen wide
pub const RESET_CSS: &str = r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

html {
    font-size: 1rem;
}

body {
    container-type: inline-size;    
}

img {
    display: block;
    width:100%;
}

a {
    all: unset;
    display: inline-block;
    cursor: pointer;
}

input {
    display: block;
    width:100%;
}

button{
    cursor: pointer;
    color:inherit;
    border:none;
    font:inherit;
}

span, strong, label{
    display: inline-block;
}
"#;

