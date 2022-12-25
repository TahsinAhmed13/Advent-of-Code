import Data.Char
import qualified Data.Map as Map 
import Data.Maybe

parse input = 
    let h = length input
        w = maximum $ map length input
        inRange (y, x) = x <= (length $ input !! (y-1))
        coords = filter inRange [(y, x) | y <- [1..h], x <- [1..w]]
        coords' = filter (\(y, x) -> ((input !! (y-1)) !! (x-1)) /= ' ') coords
        elements = map (\(y, x) -> (input !! (y-1)) !! (x-1)) coords'
    in  Map.fromList $ zip coords' elements

turn 'R' d = (d+1) `mod` 4
turn 'L' d = (d+3) `mod` 4

slice grid (y, x) d
    | even d = Map.filterWithKey (\(y', _) _ -> y == y') grid
    | otherwise = Map.filterWithKey (\(_, x') _ -> x == x') grid

step grid (y, x) d = 
    let diff = [1, 1, -1, -1] !! d
        part = slice grid (y, x) d
        n = Map.size part
        ind = Map.findIndex (y, x) part
        ind' = (ind+diff+n) `mod` n
    in  fst $ Map.elemAt ind' part

move grid 0 (y, x) d = (y, x)
move grid n (y, x) d = 
    let (y', x') = step grid (y, x) d
        wall = (fromMaybe ' ' $ Map.lookup (y', x') grid) == '#'
    in  if wall then (y, x)
        else move grid (n-1) (y', x') d

simulate grid dirs (y, x) d 
    | null dirs = 1000*y+4*x+d
    | isDigit $ head dirs = 
        let n = (read $ takeWhile isDigit dirs) :: Int
            (y', x') = move grid n (y, x) d
            rest = dropWhile isDigit dirs
        in  simulate grid rest (y', x') d
    | otherwise = 
        let d' = turn (head dirs) d
        in  simulate grid (tail dirs) (y, x) d'

part1 input = 
    let grid = parse $ takeWhile (not . null) input 
        dirs = last input
        (sy, sx) = fst $ Map.findMin grid
    in  simulate grid dirs (sy, sx) 0

transition (0, y, _) 0 = ((1, y, 0), 0)
transition (0, _, x) 1 = ((2, 0, x), 1)
transition (0, y, _) 2 = ((4, 49-y, 0), 0)
transition (0, _, x) 3 = ((5, x, 0), 0)
transition (1, y, _) 0 = ((3, 49-y, 49), 2)
transition (1, _, x) 1 = ((2, x, 49), 2)
transition (1, y, _) 2 = ((0, y, 49), 2)
transition (1, _, x) 3 = ((5, 49, x), 3)
transition (2, y, _) 0 = ((1, 49, y), 3)
transition (2, _, x) 1 = ((3, 0, x), 1)
transition (2, y, _) 2 = ((4, 0, y), 1)
transition (2, _, x) 3 = ((0, 49, x), 3)
transition (3, y, _) 0 = ((1, 49-y, 49), 2)
transition (3, _, x) 1 = ((5, x, 49), 2)
transition (3, y, _) 2 = ((4, y, 49), 2)
transition (3, _, x) 3 = ((2, 49, x), 3)
transition (4, y, _) 0 = ((3, y, 0), 0)
transition (4, _, x) 1 = ((5, 0, x), 1)
transition (4, y, _) 2 = ((0, 49-y, 0), 0)
transition (4, _, x) 3 = ((2, x, 0), 0)
transition (5, y, _) 0 = ((3, 49, y), 3)
transition (5, _, x) 1 = ((1, 0, x), 1)
transition (5, y, _) 2 = ((0, 0, y), 1)
transition (5, _, x) 3 = ((4, 49, x), 3)

step2 (r, y, x) d = 
    let inRange (y, x) = 0 <= y && y < 50 && 0 <= x && x < 50 
        (dy, dx) = [(0, 1), (1, 0), (0, -1), (-1, 0)] !! d
        (y', x') = (y+dy, x+dx)
    in  if inRange (y', x') then ((r, y', x'), d)
        else transition (r, y, x) d

fromRegion (r, y, x) = 
    let (dy, dx) = [(1, 51), (1, 101), (51, 51), (101, 51), (101, 1), (151, 1)] !! r
    in  (y+dy, x+dx)

move2 grid 0 (r, y, x) d = ((r, y, x), d)
move2 grid n (r, y, x) d = 
    let ((r', y', x'), d') = step2 (r, y, x) d
        (ry, rx) = fromRegion (r', y', x')
        wall = (fromMaybe ' ' $ Map.lookup (ry, rx) grid) == '#'
    in  if wall then ((r, y, x), d)
        else move2 grid (n-1) (r', y', x') d'

simulate2 grid dirs (r, y, x) d 
    | null dirs = 
        let (ry, rx) = fromRegion (r, y, x)
        in  1000*ry+4*rx+d
    | isDigit $ head dirs = 
        let n = (read $ takeWhile isDigit dirs) :: Int
            ((r', y', x'), d') = move2 grid n (r, y, x) d
            rest = dropWhile isDigit dirs
        in  simulate2 grid rest (r', y', x') d'
    | otherwise = 
        let d' = turn (head dirs) d
        in  simulate2 grid (tail dirs) (r, y, x) d'
             
part2 input = 
    let grid = parse $ takeWhile (not . null) input 
        dirs = last input
    in  simulate2 grid dirs (0, 0, 0) 0

main = do 
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
