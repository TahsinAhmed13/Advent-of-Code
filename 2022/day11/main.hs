import Data.Char
import Data.List

data Monkey = Monkey [Int] (Int -> Int) (Int -> Bool) Int Int

getItems (Monkey items _ _ _ _) = items

parseMonkey input =
    let items = map (read . filter isDigit) . drop 2 . words $ input !! 1
        sym = (!!4) . words $ input !! 2
        x = (!!5) . words $ input !! 2
        op = if x == "old" then (^2) else (if sym == "*" then (*(read x)) else (+(read x)))
        k = read . (!!3) . words $ input !! 3
        test = (==0) . (`mod` k)
        a = read . (!!5) . words $ input !! 4
        b = read . (!!5) . words $ input !! 5
    in  Monkey items op test a b

parse input = 
    let first = takeWhile (not . null) input
        second = dropWhile (not . null) input
    in  if null second then [parseMonkey first] 
        else (parseMonkey first):(parse $ tail second)

getMod input = 
    let m = read . (!!3) . words $ input !! 3
        rest = dropWhile (not . null) input 
    in  if null rest then m else m * (getMod $ tail rest)

addItem x n monkeys = 
    let (first, second) = splitAt n monkeys
        (Monkey items op test a b) = head second
        items' = items ++ [x]
    in  first ++ (Monkey items' op test a b):(tail second)

throwItem n monkeys = 
    let (first, second) = splitAt n monkeys
        (Monkey (x:xs) op test a b) = head second
        monkeys' = first ++ (Monkey xs op test a b):(tail second)
        y = op x
    in  if test y then addItem y a monkeys' else addItem y b monkeys'
        
throwAll n monkeys
    | null items = monkeys
    | otherwise = throwAll n $ throwItem n monkeys
    where (Monkey items _ _ _ _) = monkeys !! n

simulate 0 monkeys = replicate (length monkeys) 0
simulate n monkeys = 
    let (count, next) = foldl (\(cnt, acc) i -> (cnt ++ [length . getItems $ acc !! i], throwAll i acc)) ([], monkeys) [0..(length monkeys)-1]
    in  zipWith (+) count $ simulate (n-1) next 

part1 input = 
    let monkeys = parse input
        monkeys' = map (\(Monkey items op test a b) -> Monkey items ((`div` 3) . op) test a b) monkeys
        (x:y:_) = sortBy (flip compare) $ simulate 20 monkeys'
    in  x*y 

part2 input =
    let m = getMod input
        monkeys = parse input
        monkeys' = map (\(Monkey items op test a b) -> Monkey items ((`mod` m) . op) test a b) monkeys
        (x:y:_) = sortBy (flip compare) $ simulate 10000 monkeys'
    in  x*y

main = do 
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
