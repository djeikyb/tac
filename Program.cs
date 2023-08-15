var lines = new List<string>();
while (Console.ReadLine() is { } s)
{
    lines.Add(s);
}

lines.Reverse();
Console.WriteLine(string.Join(Environment.NewLine, lines));
