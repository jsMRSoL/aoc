file = open("input.txt", "r")
input = file.read()

input_list = input.split('\n')

for line in input_list: # loop through all letters in input line
    nums = []
    for letter in line: # loop through all characters in input line
        if letter.isnumeric():
            nums.append(letter) # append to our list of integer characters

# part 1
sum = 0 # create our sum integer
for line in input_list: # loop through each line in our puzzle input
    nums = [] # craate empty list that we will use to track all integers in list
    for letter in line: # loop through all characters in input line
        if letter.isnumeric():
            nums.append(letter) # append to our list of integer characters

    # concat the first and last characters of our list and add their integer representation to our sum
    if nums:
        print(nums[0] + nums[-1])
        sum += int(nums[0] + nums[-1])

print(sum) # print solution to part 1
