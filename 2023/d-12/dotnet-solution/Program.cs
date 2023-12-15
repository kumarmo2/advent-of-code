namespace dotnet_solution;
using System.Collections.Generic;
using System.Runtime.InteropServices;

class Program
{
    static void Main(string[] args)
    {
        Console.WriteLine("Hello, World!");
        var input = File.ReadAllText("../input-prod.txt");
        Console.WriteLine(Part1(input));
    }


    private static uint Part1(string input)
    {
        var lines = input.Split("\n", StringSplitOptions.TrimEntries);
        uint total = 0;
        foreach (var line in lines)
        {
            if (line.Length == 0)
            {
                continue;
            }
            var index = line.IndexOf(' ');
            ReadOnlySpan<char> lineSpan = line.Trim();
            var fields = lineSpan.Slice(0, index);
            var numsStrings = lineSpan.Slice(index + 1);
            var nums = new List<uint>();

            while (numsStrings.Length > 0)
            {
                int length = 0;
                while (length < numsStrings.Length && numsStrings[length] != ',')
                {
                    length++;
                }
                var numString = numsStrings.Slice(0, length);
                nums.Add(uint.Parse(numString));
                if (length >= numsStrings.Length)
                {
                    break;
                }
                numsStrings = numsStrings.Slice(length + 1);
            }
            //
            // Console.WriteLine($"fields: {fields}, length: {fields.Length}");
            // Console.Write("nums: ");
            // foreach (var num in nums)
            // {
            //     Console.Write(num + " ");
            // }
            // Console.WriteLine();
            var x = CollectionsMarshal.AsSpan(nums).Slice(0);
            total = total + Rec(fields, x);

        }
        return total;
    }

    static uint Rec(ReadOnlySpan<char> fields, ReadOnlySpan<uint> nums)
    {
        if (nums.Length == 0)
        {
            if (fields.Contains('#'))
            {
                return 0;
            }
            return 1;
        }
        if (fields.Length == 0 && nums.Length > 0)
        {
            return 0;
        }

        if (fields.StartsWith("."))
        {
            return Rec(fields.Slice(1), nums);
        }

        if (fields.StartsWith("#"))
        {
            var count = 0;
            var localFields = fields;

            while (count < nums[0])
            {
                if (localFields.StartsWith("#") || localFields.StartsWith("?"))
                {
                    count++;
                    localFields = localFields.Slice(1);
                }
                else
                {
                    break;
                }
            }
            if (count < nums[0])
            {
                return 0;
            }

            if (localFields.StartsWith("#"))
            {
                return 0;
            }
            if (localFields.StartsWith("?"))
            {
                return Rec(localFields.Slice(1), nums.Slice(1));
            }
            return Rec(localFields, nums.Slice(1));

        }

        if (fields.StartsWith("?"))
        {

            var count1 = Rec(fields.Slice(1), nums);

            var count2 = 0;
            var localFields = fields;
            while (count2 < nums[0])
            {
                if (localFields.StartsWith("?") || localFields.StartsWith("#"))
                {
                    count2++;
                    localFields = localFields.Slice(1);
                }
                else
                {
                    break;
                }
            }
            if (count2 < nums[0])
            {
                return count1;
            }
            if (localFields.StartsWith("#"))
            {
                return count1;
            }
            if (localFields.StartsWith("?"))
            {
                return Rec(localFields.Slice(1), nums.Slice(1)) + count1;
            }
            return Rec(localFields, nums.Slice(1)) + count1;
        }
        // Console.WriteLine($"fields: {fields[0]}, length: {fields.Length}");
        // Console.WriteLine($"nums: {nums}");
        throw new NotImplementedException();
    }
}
