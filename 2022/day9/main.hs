import qualified Data.Set as Set

parse input = 
    let tokens = map words input
    in  map (\[x, y] -> (head x, read y :: Int)) tokens 

next (x, y) 'L' = (x-1, y)
next (x, y) 'R' = (x+1, y)
next (x, y) 'U' = (x, y+1)
next (x, y) 'D' = (x, y-1)

follow (hx, hy) (tx, ty) = 
    let dx = hx-tx
        dy = hy-ty
        tx' = if dx /= 0 then tx+(dx `div` (abs dx)) else tx
        ty' = if dy /= 0 then ty+(dy `div` (abs dy)) else ty
    in  if abs dx < 2 && abs dy < 2 then (tx, ty)
        else (tx', ty')

step (hx, hy) (tx, ty) dir = 
    let (hx', hy') = next (hx, hy) dir
        (tx', ty') = follow (hx', hy') (tx, ty)
    in  ((hx', hy'), (tx', ty'))

visited (hx, hy) (tx, ty) [] = Set.empty
visited (hx, hy) (tx, ty) (x:xs) = 
    let ((hx', hy'), (tx', ty')) = step (hx, hy) (tx, ty) (fst x) 
        xs' = if snd x > 1 then (fst x, (snd x)-1):xs else xs
    in  Set.insert (tx, ty) $ visited (hx', hy') (tx', ty') xs'        

step2 coords dir = 
    let (hx', hy') = next (last coords) dir
    in  foldr (\x acc -> (follow (head acc) x):acc) [(hx', hy')] (init coords)

visited2 coords [] = Set.empty
visited2 coords (x:xs) = 
    let coords' = step2 coords (fst x)
        xs' = if snd x > 1 then (fst x, (snd x)-1):xs else xs
    in  Set.insert (head coords) $ visited2 coords' xs'

part1 input = 
    let moves = parse input
    in  Set.size $ visited (0, 0) (0, 0) moves

part2 input = 
    let moves = parse input
        coords = replicate 10 (0, 0)
    in  Set.size $ visited2 coords moves

main = do
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
