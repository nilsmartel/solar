type direction = left | right

type tree =
    | leaf
    | node(v: int, left: tree, right: tree)

function depth(tree) -> int;
function depth(leaf) = 0;
function depth(node(_, l, r)) = 1 +  max (depth l) (depth r);



function depth(t: tree) -> int =
        is leaf: 0
        is node(_, l, r): max (depth l) (depth r)



let sumtree = match(tree)
    is leaf then 0
    is node(v, l, r) then v + sumtree l + sumtree r


#   `match` keyword
#
#   The `match` keyword automatically _evalutes_ `is` statements
#
#   These three are behaving very similiar
#
#   ( FIRST )
#
#   let f: function(direction) -> int =
#       is left then 0
#       is right then 1
#
#   f(something)
#
#
#   ( SECOND )
#
#   something | is left then 0 is right then 1
#
#
#   ( THIRD }
#
#   ( is left then 0 is right then 1) something
#
#
#   Result: REMOVE  match KEYWORD


Open Discussion:

    is <expr> then

        vs

    is <expr> :

is left ( "left" )
or is right ( 1234567 | 
        is x: x & 1 == 0 ( "even")
        else ( "uneven" )
    )
