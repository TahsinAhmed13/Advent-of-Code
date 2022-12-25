import qualified Data.Map as Map

parse _ [] = Map.empty
parse stack (x:xs)
    | cmds !! 1 == "cd" && cmds !! 2 == "/" = parse ["/"] xs
    | cmds !! 1 == "cd" && cmds !! 2 == ".." = parse (tail stack) xs
    | cmds !! 1 == "cd" = parse ((cmds!!2):stack) xs
    | otherwise = 
        let files = takeWhile (\x -> (head x) /= '$') xs
            rest = dropWhile (\x -> (head x) /= '$') xs
        in  Map.insert stack files $ parse stack rest
    where cmds = words x

size stack dict = 
    let fileSize file = 
            let tokens = words file
            in if (tokens !! 0) == "dir" then size ((tokens!!1):stack) dict
               else read (tokens !! 0) :: Int
        Just subdirs = Map.lookup stack dict
    in  sum $ map fileSize subdirs

part1 input =
    let dict = parse [] input
    in  sum . filter (<=100000) $ map (flip size $ dict) (Map.keys dict) 

part2 input = 
    let dict = parse [] input
        total = 70000000
        used = size ["/"] dict
        free = total - used
        need = 30000000 - free
    in  minimum . filter (>=need) $ map (flip size $ dict) (Map.keys dict)
        

main = do
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
