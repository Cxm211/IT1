namespace
{
  namespace
  {
    template < template < typename > class Atom > struct SequentialThreadId
    {
      Atom < unsigned >prevId;
    };
}}