use std::os::unix::prelude::*;
use std::fs::File;
use std::os::raw::c_char;
use std::os::raw::c_void;
use dwarf_bindings::*;
use std::ptr;
use std::ffi::CString;

fn dwarf_error() -> *mut *mut Struct_Dwarf_Error_s {
  let mut x: Dwarf_Error = ptr::null::<Struct_Dwarf_Error_s>() as Dwarf_Error;
  &mut x as *mut *mut Struct_Dwarf_Error_s
}

fn print_die_data(dbg: Dwarf_Debug, print_me: Dwarf_Die, level: u32)
{
    let error_ptr = dwarf_error();
    let tag: Dwarf_Half = 0;
    let mut tagname = ptr::null::<c_char>() as *mut c_char;
    let mut name = ptr::null::<c_char>() as *mut c_char;
    unsafe {
        let res = dwarf_diename(print_me, &mut name as *mut *mut c_char,error_ptr);
        println!("{:?}", CString::from_raw(name));
    }
/*
    char *name = 0;
    const char *tagname = 0;
    if(res == DW_DLV_ERROR) {
        printf("Error in dwarf_diename , level %d \n",level);
        exit(1);
    }
    if(res == DW_DLV_NO_ENTRY) {
        return;
    }
    res = dwarf_tag(print_me,&tag,&error);
    if(res != DW_DLV_OK) {
        printf("Error in dwarf_tag , level %d \n",level);
        exit(1);
    }
    res = dwarf_get_TAG_name(tag,&tagname);
    if(res != DW_DLV_OK) {
        printf("Error in dwarf_get_TAG_name , level %d \n",level);
        exit(1);
    }
    printf("<%d> tag: %d %s  name: %s\n",level,tag,tagname,name);
    dwarf_dealloc(dbg,name,DW_DLA_STRING);
    */
}


fn get_die_and_siblings(dbg: Dwarf_Debug, in_die: Dwarf_Die, in_level: u32)
{
    let child = ptr::null::<Struct_Dwarf_Die_s>() as Dwarf_Die;
    let cur_die = in_die;
    let mut error: Dwarf_Error = ptr::null::<Struct_Dwarf_Error_s>() as Dwarf_Error;
    let mut res = DW_DLV_ERROR;
    print_die_data(dbg,in_die,in_level);
/*

    for(;;) {
        Dwarf_Die sib_die = 0;
        res = dwarf_child(cur_die,&child,&error);
        if(res == DW_DLV_ERROR) {
            printf("Error in dwarf_child , level %d \n",in_level);
            exit(1);
        }
        if(res == DW_DLV_OK) {
            get_die_and_siblings(dbg,child,in_level+1);
        }
        /* res == DW_DLV_NO_ENTRY */
        res = dwarf_siblingof(dbg,cur_die,&sib_die,&error);
        if(res == DW_DLV_ERROR) {
            printf("Error in dwarf_siblingof , level %d \n",in_level);
            exit(1);
        }
        if(res == DW_DLV_NO_ENTRY) {
            /* Done at this level. */
            break;
        }
        /* res == DW_DLV_OK */
        if(cur_die != in_die) {
            dwarf_dealloc(dbg,cur_die,DW_DLA_DIE);
        }
        cur_die = sib_die;
    }
    return;
    */
}
fn read_cu_list(dbg: Dwarf_Debug) {
    let mut cu_header_length: Dwarf_Unsigned  = 0;
    let mut version_stamp: Dwarf_Half  = 0;
    let mut abbrev_offset: Dwarf_Unsigned  = 0;
    let mut address_size: Dwarf_Half  = 0;
    let mut next_cu_header: Dwarf_Unsigned  = 0;
    let mut error: Dwarf_Error = ptr::null::<Struct_Dwarf_Error_s>() as Dwarf_Error;

    let i = 0;
    while true {
      let no_die: Dwarf_Die  = ptr::null::<Struct_Dwarf_Die_s>() as Dwarf_Die;
      let mut cu_die: Dwarf_Die  = ptr::null::<Struct_Dwarf_Die_s>() as Dwarf_Die;
      unsafe {
        let mut res = DW_DLV_ERROR;
        res = dwarf_next_cu_header(dbg,
            &mut cu_header_length,
            &mut version_stamp as *mut Dwarf_Half,
            &mut abbrev_offset as *mut Dwarf_Unsigned, 
            &mut address_size as *mut Dwarf_Half,
            &mut next_cu_header as *mut Dwarf_Unsigned,
            &mut error as *mut *mut Struct_Dwarf_Error_s);
        if res == DW_DLV_ERROR {
            panic!("Error in dwarf_next_cu_header\n");
        }
        if res == DW_DLV_NO_ENTRY {
            println!("done");
            return;
        }
        println!("{}, {}, {}", cu_header_length, address_size, next_cu_header);
        res = dwarf_siblingof(dbg,no_die,
        &mut cu_die as *mut Dwarf_Die ,
        &mut error as *mut *mut Struct_Dwarf_Error_s
        );
        if(res == DW_DLV_ERROR) {
            panic!("Error in dwarf_siblingof on CU die \n");
        }
        if(res == DW_DLV_NO_ENTRY) {
            /* Impossible case. */
            panic!("no entry! in dwarf_siblingof on CU die \n");
        }
        get_die_and_siblings(dbg,cu_die,0);
      }
    }

/*
    for(;;++cu_number) {
        int res = DW_DLV_ERROR;
        res = dwarf_next_cu_header(dbg,&cu_header_length,
            &version_stamp, &abbrev_offset, &address_size,
            &next_cu_header, &error);
        if(res == DW_DLV_ERROR) {
            printf("Error in dwarf_next_cu_header\n");
            exit(1);
        }
        if(res == DW_DLV_NO_ENTRY) {
            /* Done. */
            return;
        }
        /* The CU will have a single sibling, a cu_die. */
        res = dwarf_siblingof(dbg,no_die,&cu_die,&error);
        if(res == DW_DLV_ERROR) {
            printf("Error in dwarf_siblingof on CU die \n");
            exit(1);
        }
        if(res == DW_DLV_NO_ENTRY) {
            /* Impossible case. */
            printf("no entry! in dwarf_siblingof on CU die \n");
            exit(1);
        }
        get_die_and_siblings(dbg,cu_die,0);
        dwarf_dealloc(dbg,cu_die,DW_DLA_DIE);
    }
*/
}



pub fn do_everything() {
    let mut dbg: Dwarf_Debug = ptr::null::<Struct_Dwarf_Debug_s>() as Dwarf_Debug;
    let errhand: Dwarf_Handler = None;
    let error_ptr = dwarf_error();
    let errarg: Dwarf_Ptr = ptr::null::<c_void> as *mut c_void;
    let file = match File::open("/home/bork/.rbenv/versions/2.1.6/bin/ruby") {
      Err(why) => panic!("couldn't open file sryyyy"),
      Ok(file) => file,
    };
    let fd  = file.as_raw_fd() as ::std::os::raw::c_int;
    unsafe {
      let res = dwarf_init(
        fd, 0, // 0 means read
        errhand,
        errarg,
        &mut dbg as *mut *mut Struct_Dwarf_Debug_s,
        error_ptr);
        if res != DW_DLV_OK {
            panic!("Giving up, cannot do DWARF processing\n");
        }
    };
    read_cu_list(dbg);
    /*
    int res = DW_DLV_ERROR;
    Dwarf_Error error;
    res = dwarf_init(fd,DW_DLC_READ,errhand,errarg, &dbg,&error);

    read_cu_list(dbg);
    res = dwarf_finish(dbg,&error);
    if(res != DW_DLV_OK) {
        printf("dwarf_finish failed!\n");
    }
    */
}