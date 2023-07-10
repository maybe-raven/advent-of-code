let max3 hd (a, b, c) =
  if hd > a then (hd, a, b)
  else if hd > b then (a, hd, b)
  else if hd > c then (a, b, hd)
  else (a, b, c)

let rec input_line_seq () =
  match In_channel.input_line In_channel.stdin with
  | None -> Seq.Nil
  | Some line -> Seq.Cons (line, input_line_seq)

let rec sum_segment ?(acc = 0) = function
  | Seq.Nil -> (acc, Seq.Nil)
  | Seq.Cons (line, next) -> (
      match int_of_string_opt line with
      | None -> (acc, next ())
      | Some x -> sum_segment (next ()) ~acc:(x + acc))

let solutionate input =
  let rec aux acc seq =
    let sum, seq = sum_segment seq in
    match seq with
    | Seq.Nil -> acc
    | Seq.Cons (_, _) as node -> aux (max3 sum acc) node
  in
  aux (0, 0, 0) input

let () =
  let a, b, c = input_line_seq () |> solutionate in
  print_int (a + b + c);
  print_newline ()
