use classfile::attribute_info::AttributeInfo;
use classfile::member_info::MemberInfo;
use rtda::heap::class_member::ClassMember;

pub struct Method {
    class_member: ClassMember,
    pub max_locals: usize,
    pub max_stack: usize,
    pub code: Vec<u8>,
}

impl Method {
    pub fn new(member_info: MemberInfo) -> Method {
        let class_member = ClassMember::new(&member_info);
        let code_attribute = member_info.code_attribute();
        match code_attribute {
            Some(AttributeInfo::Code {
                max_stack,
                max_locals,
                code,
                ..
            }) => Method {
                class_member,
                max_stack: *max_stack as usize,
                max_locals: *max_locals as usize,
                //todo:fix here
                code: code.to_vec(),
            },
            None => Method {
                class_member,
                max_stack: 0,
                max_locals: 0,
                code: Vec::new(),
            },
            _ => panic!(),
        }
    }
    pub fn is_static(&self) -> bool {
        self.class_member.is_static()
    }

    pub fn name(&self) -> &str {
        &self.class_member.name
    }

    pub fn descriptor(&self) -> &str {
        &self.class_member.descriptor
    }
}
