# Classes
class Bank
  def initialize(battery_str)
    # Instance Variables
    @bank_values = Array.new

    # Parse the battery string and turn it into a list of integers
    battery_str.each_char do |c|
      # Add the parsed value of the character into the list
      @bank_values.push(c) if c != "\n"
    end

    # Initialize the max voltage and set it
    @max_voltage = 0
    set_bank_voltage
  end

  def set_bank_voltage_step_one
    # Loop through the bank and determine the highest voltage from the bank
    @bank_values.each_with_index do |b,i|
      @bank_values[(i+1)..].each do |c|
        # Combine the two values and convert into an integer
        voltage = (b + c).to_i
        @max_voltage = voltage if voltage > @max_voltage
      end
    end
  end

  def set_bank_voltage
    # initialize variables for the algorithm
    leeway = 3 # how many digits we can remove
    index = 0
    digits = Array.new

    p @bank_values[index, index+leeway]

    # while leeway != 0
    #   # While we have some leeway, choose the largest in a group of 4 digits
    #   if index + leeway >= @bank_values.size
    #     @bank_values[index].each_char do |c|
    #       digits.push(c)
    #     end
    #     break
    #   end
    #
    #   max_index_four = @bank_values[index..index+3].each_with_index.max[1]
    #   if max_index_four != index
    #     leeway -= max_index_four
    #     index = max_index_four
    #     digits.push(@bank_values[index])
    #   else
    #     digits.push(@bank_values[index])
    #     index += 1
    #   end
    # end

    # puts digits
    @max_voltage = 0
  end

  def to_s
    "Max Voltage: #{@max_voltage}, Bank Values: #{@bank_values}"
  end

  # Getters
  attr_reader :max_voltage

end

class Battery
  def initialize
    @banks = Array.new
  end

  def add_bank(bank)
    @banks.push(bank)
  end

  def get_total_voltage
    total_voltage = 0
    @banks.each do |b|
	    # puts b.max_voltage
      total_voltage += b.max_voltage
    end

    total_voltage
  end

end

#################################################################################
# Create functions
def work_with_file(testing=false)
  # Create variable for file_path depending on if we are doing test_input or not
  file_path = if !testing
    "input"
  else
    "input3"
  end

  # Open the file and print contents to terminal
  # Check if file exists
  if File.file?(file_path)
    # Initialize values
    battery = Battery.new

    # Open the file within a foreach to get contents of it
    File.foreach(file_path) do |line|
      # Make sure we are not working with an empty line
      next if line.strip.empty?

      # Create a bank
      bank = Bank.new(line)

      # Add a bank to the battery
      battery.add_bank(bank)
    end

    # Get the total voltage from every banks max voltage
    puts battery.get_total_voltage
  end
end

work_with_file(true)