export class Person {
    id: number
    name: string
    vote: number
    created_at: Date
    updated_at: Date

    constructor(
        id: number, 
        name: string, 
        vote: number, 
        created_at: Date, 
        updated_at: Date
    ) {
        this.id = id;
        this.name = name;
        this.vote = vote;
        this.created_at = created_at,
        this.updated_at = updated_at
    } 
}