type play = Rock | Paper | Scissors
type winnage = Win | Loss | Draw

let play_of_char = function
  | 'A' | 'X' -> Some Rock
  | 'B' | 'Y' -> Some Paper
  | 'C' | 'Z' -> Some Scissors
  | _ -> None

let winnage_of_char = function
  | 'X' -> Some Loss
  | 'Y' -> Some Draw
  | 'Z' -> Some Win
  | _ -> None

let calc_play = function
  | Rock, Win | Scissors, Loss | Paper, Draw -> Paper
  | Rock, Loss | Paper, Win | Scissors, Draw -> Scissors
  | Rock, Draw | Scissors, Win | Paper, Loss -> Rock

let score_of_play = function Rock -> 1 | Paper -> 2 | Scissors -> 3
let score_of_winnage = function Win -> 6 | Draw -> 3 | Loss -> 0

let parse_line line =
  try
    let char0 = String.get line 0 |> play_of_char in
    let char1 = String.get line 2 |> winnage_of_char in
    match (char0, char1) with
    | Some a, Some b -> Some (a, b)
    | None, Some _ | Some _, None | None, None -> None
  with Invalid_argument _ -> None

let rec input_line_seq () =
  match In_channel.input_line In_channel.stdin with
  | None -> Seq.Nil
  | Some line -> Seq.Cons (line, input_line_seq)

let rec solutionate ?(acc = 0) = function
  | Seq.Nil -> Ok acc
  | Seq.Cons (head, next) -> (
      match parse_line head with
      | None -> Error ("invalid input: " ^ head)
      | Some (their_play, goal) ->
          let my_play = calc_play (their_play, goal) in
          let score = score_of_winnage goal + score_of_play my_play in
          solutionate (next ()) ~acc:(acc + score))

let () =
  (match input_line_seq () |> solutionate with
  | Error e -> print_endline e
  | Ok result -> print_int result);
  print_newline ()
