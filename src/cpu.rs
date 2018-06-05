
use failure::Error;
use std::string;

pub const MEMORY_MAX : usize = 4096;
pub const REGISTERS_MAX : usize = 16;
pub const GFX_MEMORY_MAX : usize = 64 * 32;
pub const STACK_MAX : usize = 16;
pub const KEYPAD_MAX : usize = 16;

pub struct Chip8 {

    ///The Chip 8 has 35 opcodes which are all two bytes long. 
    /// To store the current opcode
    opcode: u16,
    
    /// The Chip 8 has 4K memory in total
    /**
     The systems memory map:
        0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
        0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
        0x200-0xFFF - Program ROM and work RAM
     **/
    memory: [u8; MEMORY_MAX],

    /// CPU registers: 
    /// The Chip 8 has 15 8-bit general purpose registers named V0,V1 up to VE. 
    /// The 16th register is used  for the ‘carry flag’. 
    registers: [u8; REGISTERS_MAX],

    /// Index register I 
    index_register : u8,

    /// program counter (pc) which can have a value from 0x000 to 0xFFF
    pc : u8,


    /// The graphics system: The chip 8 has one instruction that draws sprite to the screen. 
    /// Drawing is done in XOR mode and if a pixel is turned off as a result of drawing, the VF register is set. 
    /// This is used for collision detection.
    /// The graphics of the Chip 8 are black and white and the screen has a total of 2048 pixels (64 x 32). 
    /// This can easily be implemented using an array that hold the pixel state (1 or 0):
    gfx : [u8; GFX_MEMORY_MAX],

    /// draw flag: set display needs updating
    draw_flag : bool,

    /// Interrupts and hardware registers. The Chip 8 has none, but there are two timer registers that count 
    /// at 60 Hz. When set above zero they will count down to zero.
    /// Delay timer register
    delay_timer : u8, 

    /// Sound timer register
    sound_timer : u8,

    /// Stack 
    stack : [u16; STACK_MAX],

    /// Stack Pointer
    sp : u16,

    /// Finally, the Chip 8 has a HEX based keypad (0x0-0xF), 
    /// Keypad array to store the current state of the key.
    keypad : [u8;KEYPAD_MAX]

}


impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            opcode : 0,
            memory : [0;MEMORY_MAX],
            registers: [0;REGISTERS_MAX],
            index_register : 0,
            pc : 0x000,
            gfx : [0; GFX_MEMORY_MAX],
            draw_flag : false,
            delay_timer : 0,
            sound_timer : 0,
            stack : [0;STACK_MAX],
            sp : 0,
            keypad: [0;KEYPAD_MAX]
        }
    }

    fn setup_gfx(&mut self) {

    }

    fn setup_input(&mut self) {

    }

    fn initialize(&mut self) {

    }

    fn load_program(&mut self, rom_filename: &str) -> Result<(), Error> {
        Ok(())
    }
    /// Boot the CHIP8 System
    pub fn boot(&mut self, rom_filename : &str) -> Result<(), Error> {
        // Set up render system and register input callbacks
        self.setup_gfx();
        self.setup_input();
 
        // Initialize the Chip8 system and load the program into the memory  
        self.initialize();
        self.load_program(rom_filename)?;
 
        Ok(())
    }

    pub fn emulate_cycle(&mut self) {

    }

    pub fn draw_graphics(&mut self) {

    }

    pub fn set_keys(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Error> {
        // Emulation loop
        loop
        {
            // Emulate one cycle
            self.emulate_cycle();
        
            // If the draw flag is set, update the screen
            if self.draw_flag {
                self.draw_graphics();
            }
            
        
            // Store key press state (Press and Release)
            self.set_keys()?;	
        } 

        Ok(())
    }
}