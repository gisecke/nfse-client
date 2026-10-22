//mostrar e oculcar os campos cnpj e cpf
        const inscricaoMunicipal = document.getElementById('im');
console.log(inscricaoMunicipal);
        function mascaraInscricaoMunicipal(valor) {
            return valor
            .replace(/\D/g, ''); // Remove tudo o que não é dígito
        }

        if (inscricaoMunicipal != null) {

            inscricaoMunicipal.addEventListener('input', (e) => {
                e.target.value = mascaraInscricaoMunicipal(e.target.value);
            });
        }

//         formata CNPJ e CPF dependendo da quantidade de dígitos
document.addEventListener('input', function (e) {
    if (e.target && e.target.id === 'docTomador') {
        let input = e.target;
        let value = input.value.replace(/\D/g, '');

        if (value.length > 14) {
            value = value.substring(0, 14);
        }

        if (value.length <= 11) {
            // Formata CPF: 000.000.000-00
            value = value.replace(/(\d{3})(\d)/, '$1.$2');
            value = value.replace(/(\d{3})(\d)/, '$1.$2');
            value = value.replace(/(\d{3})(\d{1,2})$/, '$1-$2');
        } else {
            // Formata CNPJ: 00.000.000/0000-00
            value = value.replace(/^(\d{2})(\d)/, '$1.$2');
            value = value.replace(/^(\d{2})\.(\d{3})(\d)/, '$1.$2.$3');
            value = value.replace(/\.(\d{3})(\d)/, '.$1/$2');
            value = value.replace(/(\d{4})(\d{1,2})$/, '$1-$2');
        }

        input.value = value;
    }

    if (e.target && e.target.id === 'cep') {
        let input = e.target;
        let value = input.value.replace(/\D/g, '');

        if (value.length > 8) {
            value = value.substring(0, 8);
        }

        // Formata CEP: 00000-000
        if (value.length > 5) {
            value = value.replace(/^(\d{5})(\d)/, '$1-$2');
        }

        input.value = value;
    }
});
        