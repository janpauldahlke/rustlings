// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

fn main() {
    let a = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam euismod blandit dolor, sed ornare lacus sollicitudin ut. Integer in mattis arcu. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Duis mattis mattis nisl in molestie. Sed consequat fringilla sem, quis lobortis nisi convallis eu. Vivamus maximus erat eget consequat tempus. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Aliquam eget arcu vel enim malesuada dignissim. Phasellus dolor augue, pulvinar id laoreet id, euismod et ex. Aliquam bibendum, ex a fringilla congue, felis felis tempor odio, tincidunt convallis urna dui vitae odio. In euismod quam sem, quis cursus mauris tincidunt et.

    Cras sed auctor odio. In ut laoreet mi, sed suscipit lacus. Sed nibh libero, placerat vitae lacus non, dapibus semper magna. Integer sit amet nisi gravida, vehicula nulla a, pulvinar lacus. Maecenas auctor laoreet eros a bibendum. Nulla vitae vulputate lectus. Duis volutpat massa felis, non ultricies ipsum accumsan lobortis. Morbi pharetra suscipit elit eget scelerisque. Mauris ex mi, condimentum eu faucibus et, aliquam sed orci.
    
    Maecenas a urna hendrerit est consectetur bibendum et eget enim. Praesent congue vitae orci id sagittis. Sed tincidunt nisl sed elit posuere, id venenatis erat dictum. Duis sit amet mollis nunc. Nam lobortis arcu in sodales gravida. Duis rutrum vehicula massa. Vestibulum eu eros mi. Fusce vehicula massa eu lorem lobortis scelerisque. Pellentesque porta, libero ut dapibus viverra, tortor ligula consectetur felis, et tincidunt turpis augue eget elit. Donec quis pretium erat. Duis iaculis, lectus nec eleifend consequat, mauris ex tempor orci, non molestie mauris mi et velit. Integer euismod pulvinar massa ac ullamcorper. Curabitur pretium tincidunt gravida. Sed fringilla metus vitae condimentum interdum. Aenean et tempus justo. Ut ut lobortis nunc.
    
    Nam quis arcu eu urna dignissim rutrum nec sit amet turpis. Curabitur lobortis dolor ac magna tempor scelerisque ac eu nisi. Aliquam lacus felis, venenatis at nunc at, aliquam iaculis sem. Mauris feugiat vestibulum enim, nec elementum nisl. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed imperdiet velit ut odio ornare lacinia. Morbi in bibendum mi, vitae egestas velit. Proin id nulla nec libero pharetra sodales in vitae risus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Mauris eros libero, lobortis id maximus et, consequat eget ante. Nullam sed pulvinar orci. Nullam volutpat, sapien nec semper tincidunt, nibh massa luctus diam, in pellentesque justo tortor vel dolor.
    
    Praesent sit amet lobortis arcu. Nam viverra ex in ex sodales, sed congue felis posuere. Nunc sem turpis, gravida a iaculis commodo, congue ac erat. Sed lacinia at tortor eget efficitur. Ut id blandit erat. Etiam euismod, mi a convallis mattis, dolor ipsum hendrerit elit, eget aliquet ex nulla nec tortor. Sed blandit mauris non elit varius fringilla. Sed malesuada turpis eget augue sagittis, vel mattis risus ornare. Pellentesque porttitor nunc vitae mauris pulvinar, ac facilisis ex dapibus. Integer fringilla tempor tortor. Donec justo nulla, faucibus eget euismod vel, sodales vitae magna. Integer consectetur sagittis nunc, vitae euismod quam tempus.
    
    ";

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
