# HALfred
Hardware Abstraction Layer for ATMega 328 written in Rust


[CORRECTION GPIO] (Don't hesitate to remove this part)
It would be nice to have something to prevent modifying the register in an incoherent way. For example, if I do ``` let gpio = Gpio::new(50, Atmega_Port::PortB).```, it won't bug during the compilation, but it may generate a problem on your hardware.


[CORRECTION USART] (Don't hesitate to remove this part)
For the USART feature of the Atmega, you could try implementing the internal clock of the Atmega as well
For your delay, you could try using the integrated timers of your targets
Is your second target a CORTEX M3 as suggest the module's title and some comment ? Or is it a HIFIVE (SiFive FU740-C000?) as suggest your commits and some of your registers values/names ?