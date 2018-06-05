
use failure::Error;
use std::fs;
use std::path::Path;

/// Memory size
const MEMORY_MAX : usize = 4096;

/// Number of registers
const REGISTERS_MAX : usize = 16;

/// Graphics memory buffer size
const GFX_MEMORY_MAX : usize = 64 * 32;

/// Maximum stack size
const STACK_MAX : usize = 16;

/// Maximum number of keys
const KEYPAD_MAX : usize = 16;

/// Start memory address of running program
const PROGRAM_START_ADDRESS : u16 = 0x200;

/// Register index used as carry
const REGISTER_CARRY_FLAX_INDEX : usize = 0xF;

/// Maximum value a register can hold
const REGISTER_VALUE_MAX : u8 = 0xFF;

pub struct Chip8 {

    /// stores the current opcode
    /// The Chip 8 has 35 opcodes which are all two bytes long. 
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
    V: [u8; REGISTERS_MAX],

    /// Index register I 
    index_register : u8,

    /// program counter (pc) which can have a value from 0x000 to 0xFFF
    pc : u16,


    /// The graphics system: The chip 8 has one instruction that draws sprite to the screen. 
    /// Drawing is done in XOR mode and if a pixel is turned off as a result of drawing, the VF register is set. 
    /// This is used for collision detection.
    /// The graphics of the Chip 8 are black and white and the screen has a total of 2048 pixels (64 x 32). 
    /// This can easily be implemented using an array that hold the pixel state (1 or 0):
    gfx : [u8; GFX_MEMORY_MAX],

    /// draw flag: set display needs updating
    /// Only two opcodes should set this flag:
    /// 0x00E0 – Clears the screen
    /// 0xDXYN – Draws a sprite on the screen
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
            V: [0;REGISTERS_MAX],
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
        self.pc             =  PROGRAM_START_ADDRESS;   // Program counter starts at 0x200
        self.opcode         = 0;                        // Reset current opcode	
        self.index_register = 0;                        // Reset index register
        self.sp             = 0;                        // Reset stack pointer
        
        // Clear display
        unimplemented!();

        // Clear stack
        unimplemented!();

        // Clear registers V0-VF
        unimplemented!();
        
        // Clear memory
        unimplemented!();

        // Load fontset
        self.load_fontset_in_memory();
    
        // Reset timers
        unimplemented!();
    }

    fn load_fontset_in_memory(&mut self) {
        /*
        for(int i = 0; i < 80; ++i)
            memory[i] = chip8_fontset[i];		
        */
        unimplemented!();
    }
    fn load_program(&mut self, rom_filename: &str) -> Result<(), Error> {
    
        // Create a path to the desired file
        let path = Path::new(rom_filename);
        
        // load program
        let buffer = fs::read(&path)?;

        // write program in CHIP8 memory
        for (i, byte) in buffer.iter().enumerate() {
            self.memory[ PROGRAM_START_ADDRESS as usize + i as usize ] = *byte;
        }
        
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

    /// Fetch opcode from memory using pc
    fn fetch_opcode(&self) -> u16 {
        ((self.memory[self.pc as usize] as u16) << 8)  | (self.memory[(self.pc + 1) as usize]) as u16
    }

    /// set/unset carry flag
    fn carry_flag(&mut self, flag: bool) {
        self.V[REGISTER_CARRY_FLAX_INDEX] = flag as u8; //carry
    }

    /// Extract X component of current opcode
    fn get_op_x(&self) -> usize {
        ((self.opcode & 0x0F00) >> 8) as usize
    }

    /// Extract Y component of current opcode
    fn get_op_y(&self) -> usize {
        ((self.opcode & 0x00F0) >> 4) as usize
    }

    /// extract last 12 bits, commonly used are ref address in opcode
    fn get_op_nnn(&self) -> u16 {
        (self.opcode & (0x0FFF as u16))
    }

    /// 4 higher bits of current opcode
    fn get_op_major(&self) -> u16 {
        self.opcode & 0xF000
    }

    /// 4 lowest bits of current opcode
    fn get_op_lower(&self) -> u16 {
        self.opcode & (0x000F as u16)
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch opcode
        self.opcode = self.fetch_opcode();

        // Decode opcode
        match self.get_op_major() {    
            
             0xA000 => {
               // ANNN: Sets I to the address NNN
                // Execute opcode
                let operand = self.get_op_nnn() as u8; // remaining 12 bits contains address
                self.index_register = operand;
                self.pc += 2;  
             },
             0x0000 => { // 0x00E0 and 0x00EE both start with 0x0
                 match self.get_op_lower() {
                     0x0000 => {
                         // 0x00E0: Clears the screen 
                         unimplemented!();
                     },
                     0x000E => { // 0x00EE: Returns from subroutine
                        unimplemented!();
                     },
                     _ => {
                        println!("Unknown opcode: {:?}", self.opcode);
                        panic!();
                    }

                 }
             },
             0x2000 => { //0x2NNN
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
             },
             0x0004 => { //0x8XY4
                let reg_x = self.get_op_x();
                let reg_y = self.get_op_y();

                if self.V[reg_y] > (REGISTER_VALUE_MAX - self.V[reg_x]) {
                    self.carry_flag(true); //carry
                } else {
                    self.carry_flag(false);
                }
                self.V[reg_x] += self.V[reg_y];
                self.pc += 2;          
            },
            0x0033 => { //0xFX33
                let i_reg = self.index_register;
                let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
                self.memory[i_reg as usize]     = self.V[reg_x] / 100;
                self.memory[(i_reg + 1) as usize] = (self.V[reg_x] / 10) % 10;
                self.memory[(i_reg + 2) as usize] = (self.V[reg_x] % 100) % 10;
                self.pc += 2;
            }
    
             // More opcodes //
        
            // not handled
             _ => {
                println!("Unknown opcode: {:?}", self.opcode);
                panic!();
             }
            
        }  
        
        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
            
        if(self.sound_timer > 0)
        {
            if self.sound_timer == 1 {
                 println!("BEEP!");
            }
            self.sound_timer -= 1;
        }  
    }

    pub fn draw_graphics(&mut self) {
        unimplemented!();
    }

    pub fn set_keys(&mut self) -> Result<(), Error> {
        unimplemented!();
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