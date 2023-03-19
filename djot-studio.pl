:- use_module(library(os)).
:- use_module(library(http/http_server)).
:- use_module(djota/djota).

:- initialization(main).

djot_index(Path, _Request, Response) :-
    http_body(Response, file(Path)).

djot_handler(Request, Response) :-
    http_body(Request, text(Source)),
    djot(Source, Html),
    http_body(Response, text(Html)),
    http_status_code(Response, 200).

main :-
    getenv("DJOT_STUDIO_PORT", PortCs),
    number_chars(Port, PortCs),
    http_listen(Port, [
	get(static/Path, djot_index(Path)),
	post('/', djot_handler)
    ]).
