import Data.Char
import qualified Data.Map as Map
import Data.Maybe
import qualified Data.Set as Set

parseAdj [] = Map.empty
parseAdj input =
    let tokens = words $ head input
        u = tokens !! 1
        vs = map (filter isAlpha) $ drop 9 tokens
    in  Map.insert u vs $ parseAdj (tail input)

parseRates [] = Map.empty
parseRates input = 
    let tokens = words $ head input
        u = tokens !! 1
        rate = (read . filter isDigit $ tokens !! 4) :: Int
    in  Map.insert u rate $ parseRates (tail input)

dijikstra adj x  = 
    let traverse dist pq
            | Set.size pq == 0 = dist
            | otherwise = 
                let ((d, u), pq') = Set.deleteFindMin pq  
                    next = filter (`Map.notMember` dist) . fromMaybe [] $ Map.lookup u adj 
                    pq'' = foldr (\v acc -> Set.insert (d+1, v) acc) pq' next   
                    dist' = Map.insert u d dist
                in  if u `Map.member` dist then traverse dist pq'
                    else traverse dist' pq''
    in  traverse (Map.empty) (Set.singleton (0, x))

pressures m dist rates vis amt x memo = 
    let vis' = Set.insert x vis
        memo' = Map.insertWith max vis' amt memo 
        isNext (u, d) = u `Set.notMember` vis' && u `Map.member` rates && d+1 <= m
        next = filter isNext . fromMaybe [] $ Map.lookup x dist
        release (u, d) = 
            let r = fromMaybe 0 $ Map.lookup u rates
            in  r*(m-d-1)
    in  if null next then memo' 
        else foldr (\(u, d) acc -> pressures (m-d-1) dist rates vis' (amt + release (u, d)) u acc) memo' next 

part1 input = 
    let adj = parseAdj input
        rates = Map.filter (>0) $ parseRates input
        dist = Map.fromList [(u, Map.toList $ dijikstra adj u) | u <- Map.keys adj]
        memo = pressures 30 dist rates Set.empty 0 "AA" Map.empty
    in  maximum . map snd $ Map.toList memo

part2 input = 
    let adj = parseAdj input
        rates = Map.filter (>0) $ parseRates input
        dist = Map.fromList [(u, Map.toList $ dijikstra adj u) | u <- Map.keys adj]
        memo = Map.mapKeys (Set.delete "AA") $ pressures 26 dist rates Set.empty 0 "AA" Map.empty 
        configs = Map.keys memo
        compatiable s1 s2 = Set.null $ Set.intersection s1 s2
        totalPressure c1 c2 = 
            let x = fromMaybe 0 $ Map.lookup c1 memo
                y = fromMaybe 0 $ Map.lookup c2 memo
            in  x+y
    in  maximum [totalPressure c1 c2 | c1 <- configs, c2 <- configs, compatiable c1 c2] 

main = do
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
