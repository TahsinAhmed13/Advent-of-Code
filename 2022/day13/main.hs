import Data.List

data NestedList a = NestedList [NestedList a] | Value a deriving (Eq, Show)

instance (Ord a) => Ord (NestedList a) where
    compare (Value x) (Value y) = compare x y
    compare (NestedList xs) (Value y) = compare (NestedList xs) (NestedList [Value y])
    compare (Value x) (NestedList ys) = compare (NestedList [Value x]) (NestedList ys)
    compare (NestedList []) (NestedList []) = EQ
    compare (NestedList (x:xs)) (NestedList []) = GT
    compare (NestedList []) (NestedList (y:ys)) = LT
    compare (NestedList (x:xs)) (NestedList (y:ys)) = compare (x, NestedList xs) (y, NestedList ys) 

brackets ']' = 1
brackets '[' = (-1)
brackets _ = 0

parseList input
    | null input = NestedList []
    | head input /= '[' = Value (read input :: Int)
    | otherwise = 
        let input' = init $ tail input 
            process x (subLists, acc) = 
                let acc' = acc + brackets x
                in  if acc' == 1 && x == ',' then ([]:subLists, acc')
                    else ((x:(head subLists)):(tail subLists), acc')
            (subLists, _) = foldr process ([[]], 1) input'
        in  NestedList $ map parseList subLists

parse input = 
    let first = input !! 0 
        second = input !! 1
        lists = (parseList first, parseList second)
        rest = drop 2 input
    in  if null rest then [lists] 
        else lists:(parse $ tail rest)
        
part1 input = 
    let lists = parse input
        indicies = filter (\i -> let (x, y) = lists !! (i-1) in x < y) [1..length lists]
    in  sum indicies

part2 input = 
    let div1 = NestedList [NestedList [Value 2]]
        div2 = NestedList [NestedList [Value 6]]
        pairs = parse input
        lists = sort $ [div1] ++ [div2] ++ map fst pairs ++ map snd pairs
        x = (length $ takeWhile (/= div1) lists) + 1
        y = (length $ takeWhile (/= div2) lists) + 1
    in  x*y

main = do 
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
