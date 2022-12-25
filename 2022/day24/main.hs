import qualified Data.Set as Set

getPhases input = 
    let h = length input
        w = length $ input !! 0 
        start = [((y, x), d) | y <- [0..(h-1)], x <- [0..(w-1)], let d = (input!!y)!!x, d /= '.']
        next (y, x) '^' = if y == 1 then (h-2, x) else (y-1, x)
        next (y, x) 'v' = if y == h-2 then (1, x) else (y+1, x)
        next (y, x) '<' = if x == 1 then (y, w-2) else (y, x-1)
        next (y, x) '>' = if x == w-2 then (y, 1) else (y, x+1)
        step storms = map (\((y, x), d) -> if d == '#' then ((y, x), d) else (next (y, x) d, d)) storms
        (_, phases) = foldl (\(cur, acc) _ -> (step cur, (Set.fromList $ map fst cur):acc)) (start, []) [1..lcm (h-2) (w-2)]  
    in  reverse phases 

distanceTo input (sy, sx) (ey, ex) sp = 
    let h = length input
        w = length $ input !! 0
        inRange (y, x) = 0 <= y && y < h && 0 <= x && x < w 
        phases = getPhases input
        adj (y, x) p = [(y+dy, x+dx) | dy <- [(-1)..1], dx <- [(-1)..1], 
            dy == 0 || dx == 0, inRange (y+dy, x+dx), (y+dy, x+dx) `Set.notMember` (phases !! p)] 
        dijikstra vis pq =  
            let ((d, ((y, x), p)), pq') = Set.deleteFindMin pq
                vis' = Set.insert ((y, x), p) vis
                p' = (p+1) `mod` (length phases)
                pq'' = foldr (\pos acc -> if (pos, p') `Set.notMember` vis'  
                    then Set.insert (d+1, (pos, p')) acc else acc) pq' $ adj (y, x) p' 
            in  if (y, x) == (ey, ex) then (d, p) 
                else if ((y, x), p) `Set.notMember` vis then dijikstra vis' pq''
                else dijikstra vis pq'
    in  dijikstra Set.empty $ Set.singleton (0, ((sy, sx), sp)) 

part1 input = 
    let h = length input
        w = length $ input !! 0 
        start = (0, 1)
        end = (h-1, w-2)
    in  fst $ distanceTo input start end 0 

part2 input = 
    let h = length input
        w = length $ input !! 0 
        start = (0, 1)
        end = (h-1, w-2)
        (d1, p1) = distanceTo input start end 0 
        (d2, p2) = distanceTo input end start p1
        (d3, _) = distanceTo input start end p2
    in  d1+d2+d3

main = do 
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
        
