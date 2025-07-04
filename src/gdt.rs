use x86_64::VirtAddr;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static!{
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            // use stastic mut array as stack storage
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE;
            // Returning the end because stack grows downwards
            // high to low addressess
            stack_end
            // No guard flow to protect against stack overflow yet 
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selector)= {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selector { code_selector, tss_selector })
    };
}

struct Selector {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};

    GDT.0.load(); // .0 is the GDT, .1 is the selectors
    unsafe {
        CS::set_reg(GDT.1.code_selector); // Reload the the code segment
        load_tss(GDT.1.tss_selector);
    }
}