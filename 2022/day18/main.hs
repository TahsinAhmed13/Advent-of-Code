import qualified Data.Set as Set

offsets = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)]

parse input = 
    let parseLine s = 
            let x = read (takeWhile (/= ',') s) :: Int
                y = read (takeWhile (/= ',') . tail $ dropWhile (/= ',') s) :: Int
                z = read (tail . dropWhile (/= ',') . tail $ dropWhile (/= ',') s) :: Int
            in  (x, y, z)
    in  map parseLine input

adj (x, y, z) = map (\(dx, dy, dz) -> (x+dx, y+dy, z+dz)) offsets

sa cubes = 
    let open = filter (`notElem` cubes) $ concatMap adj cubes
    in  length open 

part1 input =
    let cubes = parse input
    in  sa cubes

dfs _ _ vis [] = vis
dfs cubes inRange vis ((x, y, z):pos) = 
    let vis' = Set.insert (x, y, z) vis 
        next = filter (\p -> p `Set.notMember` vis' && p `Set.notMember` cubes && inRange p) $ adj (x, y, z)
    in  dfs cubes inRange vis' (next ++ pos)

sa2 cubes = 
    let open = filter (`Set.notMember` cubes) $ concatMap adj cubes  
        xs = map (\(x, _, _) -> x) open
        ys = map (\(_, y, _) -> y) open
        zs = map (\(_, _, z) -> z) open
        minx = minimum xs
        maxx = maximum xs
        miny = minimum ys
        maxy = maximum ys
        minz = minimum zs
        maxz = maximum zs
        inRange (x, y, z) = minx <= x && x <= maxx && miny <= y && y <= maxy && minz <= z && z <= maxz
        vis = dfs cubes inRange Set.empty [(minx, miny, minz)]
        outside = filter (`Set.member` vis) open
    in  length outside

part2 input = 
    let cubes = Set.fromList $ parse input
    in  sa2 cubes
    
main = do
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
