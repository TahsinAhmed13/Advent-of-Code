fromSnafu s =
    let value '=' = (-2)
        value '-' = (-1)
        value '0' = 0
        value '1' = 1
        value '2' = 2
        (v, _) = foldr (\x (v, p) -> (v+x*p, 5*p)) (0, 1) $ map value s 
    in  v

convert 0 = []
convert x = 
    let r = x `mod` 5
        v = if r <= 2 then r else (r-5)
        x' = (x `div` 5) + (if r <= 2 then 0 else 1)
    in  v:convert x'
    
toSnafu x = 
    let value (-2) = '='
        value (-1) = '-'
        value 0 = '0'
        value 1 = '1'
        value 2 = '2' 
        xs = reverse $ convert x
    in  map value xs

part1 input =
    let x = sum $ map fromSnafu input 
    in  toSnafu x  

main = do 
    input <- getContents
    putStrLn . part1 $ lines input
