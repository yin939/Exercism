def response(hey_bob: str):
    msg = hey_bob.strip()
    question = msg.endswith('?')
    shouting = msg.isupper() and msg.isascii()
    if msg == '':
        return "Fine. Be that way!"

    if question and shouting:
        return "Calm down, I know what I'm doing!"

    if question:
        return "Sure."

    if shouting:
        return "Whoa, chill out!"

    return "Whatever."
