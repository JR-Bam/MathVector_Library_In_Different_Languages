module Mvec where

data Vector = Vector {
    size :: Int,
    components :: [Float]
} deriving (Eq)

instance Show Vector where
    show (Vector _ xs) = show xs

instance Num Vector where
    v1 + v2 | sameSize v1 v2 = Vector (size v1) (zipWith (+) (components v1) (components v2))
            | otherwise      = error "Vectors must be of same size"
    
    v1 - v2 | sameSize v1 v2 = Vector (size v1) (zipWith (-) (components v1) (components v2))
            | otherwise      = error "Vectors must be of same size"

vecFromList :: [Float] -> Vector
vecFromList [] = error "List must be nonempty."
vecFromList xs = Vector (length xs) xs

scaleVec :: Vector -> Float -> Vector
scaleVec (Vector n xs) alpha = Vector n (map (alpha *) xs)

magnitude :: Vector -> Float 
magnitude v1 = sqrt (dot v1 v1)

normalize :: Vector -> Vector
normalize v1 = scaleVec v1 (1.0 / magnitude v1)



sameSize :: Vector -> Vector -> Bool
sameSize (Vector s1 _) (Vector s2 _) = s1 == s2

dot :: Vector -> Vector -> Float
dot v1 v2 
    | sameSize v1 v2 = sum $ zipWith (*) (components v1) (components v2)
    | otherwise      = error "Vectors must be of same size"

axpy :: Float -> Vector -> Vector -> Vector
axpy alpha v1 v2 
    | sameSize v1 v2 = Vector (size v1) (zipWith (\x y -> x * alpha + y) (components v1) (components v2))
    | otherwise      = error "Vectors must be of same size"