let is_lower_ascii x = 'a' <= x && x <= 'z'
let is_upper_ascii x = 'A' <= x && x <= 'Z'

let index_of_item x =
  if is_lower_ascii x then Char.code x - Char.code 'a'
  else if is_upper_ascii x then Char.code x - Char.code 'A' + 26
  else invalid_arg ("index_of_item " ^ Char.escaped x)

let map_char_occurrence s =
  let memo = Array.make 52 false in
  String.iter (fun x -> memo.(index_of_item x) <- true) s;
  memo

let find_common s0 s1 s2 =
  let m0 = map_char_occurrence s0 in
  let m1 = map_char_occurrence s1 in
  let g x =
    let i = index_of_item x in
    if m0.(i) && m1.(i) then Some i else None
  in
  String.to_seq s2 |> Seq.find_map g |> Option.map (fun x -> x + 1)

let figure_out_group = function
  | Seq.Nil -> Ok None
  | Seq.Cons (s0, next) -> (
      match next () with
      | Seq.Nil -> Error "incomplete input"
      | Seq.Cons (s1, next) -> (
          match next () with
          | Seq.Nil -> Error "incomplete input"
          | Seq.Cons (s2, next) -> (
              match find_common s0 s1 s2 with
              | Some result -> Ok (Some (result, next))
              | None ->
                  Error
                    (String.concat "\n"
                       [ "group contains no shared item:"; s0; s1; s2 ])
              | exception Invalid_argument _ ->
                  Error
                    (String.concat "\n"
                       [ "group contains no shared item:"; s0; s1; s2 ]))))

let rec input_line_seq () =
  match In_channel.input_line In_channel.stdin with
  | None -> Seq.Nil
  | Some line -> Seq.Cons (line, input_line_seq)

let rec solutionate ?(acc = 0) input =
  match figure_out_group input with
  | Ok None -> Ok acc
  | Ok (Some (x, next)) -> solutionate (next ()) ~acc:(acc + x)
  | Error e -> Error e

let () =
  (match input_line_seq () |> solutionate with
  | Error e -> print_endline e
  | Ok result -> print_int result);
  print_newline ()
