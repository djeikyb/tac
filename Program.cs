var lines = new List<string>();
while (Console.ReadLine() is { } s)
{
    lines.Add(s);
}

for (var i = lines.Count - 1; i >= 0; i--)
{
    var ln = lines[i];
    Console.WriteLine(ln);
}

// lines.Reverse();
// Console.WriteLine(string.Join(Environment.NewLine, lines));
