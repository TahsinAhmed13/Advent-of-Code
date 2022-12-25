import Data.Char
import Data.List
import qualified Data.Set as Set

isNum c = isDigit c || c == '-'

parseLine input = 
    let tokens = words input
        sx = (read . filter isNum $ tokens !! 2) :: Int
        sy = (read . filter isNum  $ tokens !! 3) :: Int
        bx = (read . filter isNum $ tokens !! 8) :: Int
        by = (read . filter isNum $ tokens !! 9) :: Int
    in  ((sx, sy), (bx, by))

parse input = map parseLine input

dist (x1, y1) (x2, y2) = abs (x1-x2) + abs (y1-y2)

findNonBeacons y ((sx, sy), (bx, by)) = 
    let dmax = dist (sx, sy) (bx, by) - abs (sy-y)
    in  Set.fromList [(x, y) | x <- [(sx-dmax)..(sx+dmax)]]

part1 input =    
    let sensors = parse input
        nonBeacons = foldr (\x acc -> findNonBeacons 2000000 x `Set.union` acc) Set.empty sensors
    in  Set.size $ nonBeacons `Set.difference` (Set.fromList $ map snd sensors) 

isAdj ((sx1, sy1), (bx1, by1)) ((sx2, sy2), (bx2, by2)) = 
    let dmax = dist (sx1, sy1) (bx1, by1) + dist (sx2, sy2) (bx2, by2) + 1
    in  dist (sx1, sy1) (sx2, sy2) <= dmax

getAdj x ys = filter (\y -> x /= y && isAdj x y) ys

getBounds inRange ((sx, sy), (bx, by)) = 
    let dmax = dist (sx, sy) (bx, by) + 1
        getPos x
            | d == dmax = [(x, sy)]
            | otherwise = [(x, sy+dmax-d), (x, sy-dmax+d)]
            where d = abs (sx-x)
    in  concatMap (filter inRange . getPos) [(sx-dmax)..(sx+dmax)]

isOutside (x, y) ((sx, sy), (bx, by)) = 
    let dmax = dist (sx, sy) (bx, by)
        d = dist (x, y) (sx, sy)
    in  d > dmax

isOutsideAll (x, y) sensors = and $ map (isOutside (x, y)) sensors

findPos inRange sensors = 
    let candidates s = 
            let adj = getAdj s sensors
                bounds = getBounds inRange s 
            in  filter ((flip isOutsideAll) adj) bounds
    in  head . head . dropWhile null $ map candidates sensors 

part2 input = 
    let sensors = parse input
        inRange (x, y) = 0 <= x && x <= 4000000 && 0 <= y && y <= 4000000
        (x, y) = findPos inRange sensors
    in  x*4000000+y

main = do 
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
