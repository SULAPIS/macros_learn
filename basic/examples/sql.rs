use basic::sql;

fn main() {
    sql!(select * form table1 where id =10 and timetamp > 1000);
}
