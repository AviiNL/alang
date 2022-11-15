// Comments need to work, obviously
/*
    C-Style Comments
    Should also work!
*/
# Hash Comments could also be a thing!

screw_keywords = "for variables" // also, no semicolons

if screw_keywords == "for variables" then
    print("Some function invocation")
end

expression = 1

everything = if expression == 1 then "is an expression" /* else null */ end


// OPTIONAL
iterator = for i in 1..10 do "Value" + i loop

for value in iterator do
    print(value + "\n")
loop
/*
    Value 1
    Value 2
    Value 3
    Value 4
    Value 5
    Value 6
    Value 7
    Value 8
    Value 9
    Value 10
*/
