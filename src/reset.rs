//container-type to allow extender to works
//without x overflow when screen wide
//
//font-size inherit on hn is mandatory
//because the browser overide it 
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

h1, h2, h3, h4, h5, h6 {
    font-size: inherit;
}

img {
    display: block;
    width:100%;
    object-fit: cover;
}

a, a:visited, a:hover, a:active, a:focus {
    display: inline-block;
    cursor: pointer;
    text-decoration: none;
    color: inherit;
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

