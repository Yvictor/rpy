import rs2py


def test_get_price_tick_move():
    res = rs2py.get_price_tick_move(100, 5)
    assert res == 102.5


def test_get_price_between_tick():
    res = rs2py.get_price_between_tick(100, 101)
    assert res == 2


def test_get_price_between_ticks():
    expect = [49.8, 49.85, 49.9, 49.95, 50.0, 50.1, 50.2, 50.3, 50.4, 50.5]
    res = rs2py.get_price_between_ticks(49.8, 50.5)
    assert res == expect

    res = rs2py.get_price_between_ticks(50.5, 49.8)
    assert res == list(reversed(expect))
