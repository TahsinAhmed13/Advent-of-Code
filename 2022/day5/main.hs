addCrate c n stacks = 
    let (first, second) = splitAt (n-1) stacks
    in  first ++ (c:(head second)):(tail second)

removeCrate n stacks = 
    let (first, second) = splitAt (n-1) stacks
        c = head $ head second
        stacks' = first ++ (tail $ head second):(tail second)
    in  (c, stacks')

moveCrates 0 _ _ stacks = stacks
moveCrates n x y stacks = 
    let (c, stacks') = removeCrate x stacks
        stacks'' = addCrate c y stacks'
    in  moveCrates (n-1) x y stacks''

moveCrates2 n x y stacks =  
    let stacks' = moveCrates n x y stacks
        (first, second) = splitAt (y-1) stacks'
        reversed = (reverse $ take n (head second)) ++ (drop n (head second))
    in  first ++ reversed:(tail second)

parseLine line stacks = 
    let processCrate i stacks = 
            let c = head $ drop (4*i-3) line
            in  if c == ' ' then stacks else addCrate c i stacks
    in  foldr processCrate stacks [1..(length stacks)]

parseStacks input = 
    let count = read . last . filter (not . null) . words $ last input :: Int
        stacks = replicate count []
    in  foldr parseLine stacks (init input)

processCmd cmd stacks = 
    let [_, n, _, x, _, y] = words cmd
    in  moveCrates (read n) (read x) (read y) stacks

processCmd2 cmd stacks = 
    let [_, n, _, x, _, y] = words cmd
    in  moveCrates2 (read n) (read x) (read y) stacks

part1 stacks cmds = map head $ foldl (flip processCmd) stacks cmds

part2 stacks cmds = map head $ foldl (flip processCmd2) stacks cmds

main = do 
    input <- getContents
    let stacks = parseStacks . takeWhile (not . null) $ lines input
        cmds = tail . dropWhile (not . null) $ lines input
    putStrLn $ part1 stacks cmds
    putStrLn $ part2 stacks cmds
