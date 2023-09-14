// source radamsa parses xml into XP
// then performs the mutations on XP

// (i believe) XP can be thought of as a flattened XML structure -- like a linked list. each node represents some XML content
// front of the linked list is the beginning of the document and the back is the end (ie. the last close tag)

// unquote: (... (a . b) ...) -> (... a b ...) <- unpacks a cons within a list
// xp-whitespace: parser tries to get a newline or space
// xp-optwhite: parser tries to get as many consecutive newlines or spaces (possibly none)
// xp-string-delim: parser tries to get single or double quote
// xp-alnum: parser tries to return byte if it is alphanumerical (represents A-Za-z0-9 in byte form)
// xp-label: parser tries to return a sequence of alphanumerical characters (must be at least length 1)
// get-quoted-upto: ??
// xp-attr-value: 

/**
 * XP spec from Owl Radamsa
 * XP = (bytes (byte ...))         -- "nothing interesting structurally": possibly text content within nodes?
 *    | (open name attrs)          -- <foo attrs>
 *    | (open-single name attrs)   -- <foo attrs />
 *    | (close name)               -- </foo>
 *    | (tag name attrs XP)        -- <foo attrs> XP </foo> after shrubbing
 *    | (plus XP1 XP2)             -- XP1 XP2, used in mutations to generate multiple AST nodes
 */

/* (shrub nodes done)
 * nodes - XP nodes to be processed
 * done - processed XP nodes
 * 
 * only cares about closed tags -- this means that there is some open tag node that needs to be associated with it
 * if the close tag doesn't have an opening tag, skip this one and instead of adding the node to the done list,
 * add a tuple that notes its tag, the attrs and its content
 */

// list interning so that we can perform eq? on tags with the guarantee that tag identifiers and attributes are unique
// for quick comparison in parser

// byte compacting --> (bytes A) + (bytes B) = (bytes A B)

use nom::IResult;
use nom::character::complete::{ one_of, alphanumeric0, alphanumeric1 };
use nom::bytes::complete::take_while;

enum XP {
    Open(String, Vec<String>),
    OpenSingle(String, Vec<String>),
    Close(String),
    Tag(String, Vec<String>, XP),
    Plus(XP, XP)
}

fn xp_whitespace(input: &str) -> String {
    one_of(b" \n")(input)
}

fn xp_optwhite() -> String {
    take_while(xp_whitespace)
}

fn xp_string_delim(input: &str) -> String {
    one_of(b"\"\'")(input)
}

fn xp_alnum(input: &str) -> String {
    alphanumeric0(input)
}

fn xp_label(input: &str) -> String {
    alphanumeric1(input)
}

fn get_quoted_upto(input: &str, delim: &str) -> String {
    let result: Vec<XP> = vec![];
    // either get all non-delimiter bytes from current pointer
    // or there's a backward slash for escape sequences, so consume those too?
    let chars = take_while0(alt((is_not(delim))));
    // TODO: how to represent output?
}

// returns delim and the actual value
fn get_attr_string_value(input: &str) -> (String, String) {
    // consume next byte which should be a string delimiter
    let delim = xp_string_delim(input);
    let chars = get_quoted_upto(input, delim);  
    (delim, chars)
}

fn xp_attr_value(input: &str) -> String {
    // two parsing options:
    // consume a string delimiter, then everything up to the next delim
    // return any alphanumeric characters
    alt((xp_label, ))
}

fn xp_attr(input: &str) -> String {
    // skip whitespace
    // get attr label
    // skip whitespace
    // two parsing branches:
        // ="value"
        // epsilon?
}

fn xp_tag_open(input: &str) -> String {
    // get tag label
    // get tag attrs
    // skip whitespace
    // determine if open or open-single node
        // if next byte is a /, open single
    // return as open or open-single
}

fn xp_tag_open_close(input: &str) -> String {
    // skip pointer over <
    // two parsing options
        // 1. close tag 
        // skip /
        // get tag label
        // skip any whitespace
        // skip >
        // create close node
        // 2. call tag-open 
}

fn try_parse(input: &str) -> IResult<XP, Err> {
    // Passes:
    // bytes -> node list
    // connect adjacent byte nodes
    // list interning 
        // replace tags/attrs with interned equivalent, so that tags are unique and can be
        // searched and compared by parser efficiently
    // group open and close tags together: .. (open X attrs) .. (close X) .. -> (tag X attrs (.. content ..))
        // shrubbing -- iterate thru list of nodes and when encountering a close node, find its open node and create a tag node
        // to replace it 
    // collect information about seen tags and attributes
        // keep a store of tags and their associated information
}

fn parse(input: &str) -> XP {
    match try_parse(input) {
        Ok(v) => v,
        Err(e) => panic!(e),
    }
}

// mutate

fn mutate() {
    // swap nodes' positions
    // duplicate node
    // parse tree path repeated
    // a node cloned many times
    // new node is inserted into AST
}

// render to XML string (XP -> XML)
/**
 * node = bytes, tag, open, open-single, close, plus
 * bytes: render as is into string, then render rest of nodes
 * tag -> render open tag, render content, then render close tag
 * open -> <foo>
 * open-single -> <foo />
 * close -> </foo>
 * plus a b -> render a then render b
 */