def to_rna(dna_strand):
    if dna_strand == '':
        return ''

    trans = {'G': 'C', 'C': 'G', 'T': 'A', 'A': 'U'}
    rna = ''
    for c in dna_strand:
        rna += trans[c]

    return rna
