# pyright: reportIndexIssue=false,reportOperatorIssue=true
def func1(p1: float, p2: str, p3, **p4) -> None:
    var1: int = p1    # This is a type violation
    var2: str = p2    # This is allowed because the types match
    var2: int         # This is an error because it redeclares var2
    var3 = p1         # var3 does not have a declared type
    return var1       # This is a type violation
