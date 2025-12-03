# Template class for base challenges in AOC25 that require file input

# Class definitions
class Lock
  def initialize
    @value = 50
    @num_wraps = 0
  end

  def num_wraps
    @num_wraps
  end

  def value
    @value
  end

  def to_s
    "Lock -> Value: #{@value} Num Wraps: #{@num_wraps}"
  end

  def turn_lock(seq)
    # Make sure that we are working with a Sequence
    unless seq.is_a?(Sequence)
      raise ArgumentError.new("Invalid sequence for turning a lock")
    end

    puts seq
    # Let's handle some math for turning a lock left or right
    # Regardless of direction, we want to make sure that we are only working with values < 100
    while seq.value > 100
      seq.modify_value(-100)
      @num_wraps += 1
    end

    # Now let's focus on the math for turning a lock by a value < 100
    case seq.direction
    when "L"
      # We need to turn the lock to the left
      # If it ever passes 0, it's set to 99
      if (value - seq.value) < 0
        if value != 0
          @num_wraps += 1
        end
        @value = 100 - (seq.value - value)
      else
        @value -= seq.value
      end
    when "R"
      # We need to turn the lock to the right
      # If it ever passes 99, it's set to 0
      if (value + seq.value) > 99
        @value = (value + seq.value) % 100
        if @value != 0
          @num_wraps += 1
        end
      else
        @value += seq.value
      end
    else
      nil
    end
  end
end

class Sequence
  def initialize(sequence_string)
    # A Sequence String is formatted as <direction><value>
    sequence_string.each_char.with_index do |char, index|
      # If we are at the start of the string, we have the direction
      if index == 0
        @direction = char
        @value = ""
      else
        # If we are here, we should have a value
        @value += char
      end
    end
    @base = sequence_string
  end

  def to_s
    "Sequence -> Direction: #{@direction} Value: #{@value}"
  end


  # Getters
  def direction
    @direction
  end

  def value
    @value.to_i
  end

  def base
    @base
  end

  def modify_value(val)
    @value = (@value.to_i + val).to_s
  end
end

###############################################################################
# Create functions
def work_with_file(testing=false)
  # Create variable for file_path depending on if we are doing test_input or not
  if not testing
    file_path = "input"
  else
    file_path = "input2"
  end

  # Open the file and print contents to terminal
  # Check if file exists
  seq = nil
  if File.file?(file_path)
    # Create a Lock that we will use for this file
    lock = Lock.new

    # Create a counter number of times the lock is zero
    num_zero = 0

    # Open the file within a foreach to get contents of it
    File.foreach(file_path) do |line|
      # Make sure we are not working with an empty line
      if line.strip.empty?
        next
      end

      # Convert the line into a sequence
      seq = Sequence.new(line)
      puts lock
      lock.turn_lock(seq)

      if lock.value == 0
        num_zero += 1
      end
      puts lock
      puts "\n"
    end

    puts lock.num_wraps + num_zero
  end
end

work_with_file(false)
