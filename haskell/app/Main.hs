module Main where


import Mvec (vecFromList, normalize, magnitude, dot, axpy)

main :: IO ()
main = do
    let vec1 = vecFromList [23.0, 12.0, 5.0, 4.0, 5.0]
    let vec2 = vecFromList [-2.0, 4.0, 50.0, 6.0, -34.5]

    putStr "Vec1: "
    print vec1
    putStr "Vec2: "
    print vec2

    putStr "Dot Product: "
    print (dot vec1 vec2)

    putStr "AXPY with alpha = -5: "
    print (axpy (-5.0) vec1 vec2)

    putStr "Vec1 + Vec2 = "
    print (vec1 + vec2)

    putStr "Vec1 - Vec2 = "
    print (vec1 - vec2)

    putStr "Vec1 == Vec2 -> "
    print (vec1 == vec2)

    let vec5 = vecFromList [-243.0, 343.0, 1.0, 54.0, 22.0, -4.0]
    putStr "Initial Vec5: "
    print vec5

    putStr "Magnitude of Vec5: "
    print (magnitude vec5)

    let normalVec5 = normalize vec5
    putStr "Normalized Vec5: "
    print normalVec5
    putStr "Magnitude of Vec5: "
    print (magnitude normalVec5)




        
