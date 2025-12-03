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

  def set_bank_voltage
    # Loop through the bank and determine the highest voltage from the bank
    @bank_values.each_with_index do |b,i|
      @bank_values[(i+1)..].each do |c|
        # Combine the two values and convert into an integer
        voltage = (b + c).to_i
        @max_voltage = voltage if voltage > @max_voltage
      end
    end
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
    "input2"
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

work_with_file(false)